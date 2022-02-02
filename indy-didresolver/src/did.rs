use crate::error::{DidIndyError, DidIndyResult};
use regex::{Error as RegexError, Regex};

use std::str::FromStr;

//TODO: add other Did requests for SCHEMA ...
#[derive(Debug, PartialEq)]
pub enum Type {
    Nym,
}

#[derive(Debug, PartialEq)]
pub enum ObjectCodes {
    Attrib = 1,
    Schema = 2,
    ClaimDef = 3,
    RevRegDef = 4,
    RevRegEntry = 5,
}

#[derive(Debug, PartialEq)]
pub struct Schema {
    pub name: String,
    pub version: String,
    type_: u8,
}

impl Schema {
    fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            type_: ObjectCodes::Schema as u8,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ClaimDef {
    pub name: String,
    pub schema_id: String,
    type_: u8,
}

impl ClaimDef {
    fn new(name: String, schema_id: String) -> Self {
        Self {
            name,
            schema_id,
            type_: ObjectCodes::ClaimDef as u8,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RevRegDef {
    name: String,
    claim_def_name: String,
    type_: u8,
}

impl RevRegDef {
    fn new(name: String, claim_def_name: String) -> Self {
        Self {
            name,
            claim_def_name,
            type_: ObjectCodes::RevRegDef as u8,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RevRegEntry {
    rev_reg_name: String,
    claim_def_name: String,
    type_: u8,
}

impl RevRegEntry {
    fn new(rev_reg_name: String, claim_def_name: String) -> Self {
        Self {
            rev_reg_name,
            claim_def_name,
            type_: ObjectCodes::RevRegEntry as u8,
        }
    }
}

// Todo: Implement
#[derive(Debug, PartialEq)]
pub struct Attrib;

#[derive(Debug, PartialEq)]
pub enum LedgerObject {
    Schema(Schema),
    ClaimDef(ClaimDef),
    RevRegDef(RevRegDef),
    RevRegEntry(RevRegEntry),
    Attrib,
}

impl FromStr for LedgerObject {
    type Err = RegexError;

    fn from_str(input: &str) -> Result<LedgerObject, Self::Err> {
        println!("Inside LO");
        // let re = Regex::new(r"^/(SCHEMA|CLAIM_DEF)/([a-zA-Z]*)/?((?:\d\.){1,2}\d)?$").unwrap();
        let re = Regex::new(r"^/(SCHEMA|CLAIM_DEF)/([a-zA-Z0-9_:]*)/?((?:\d\.){1,2}\d)?$").unwrap();

        let captures = re.captures(input);

        if let Some(cap) = captures {
            println!("{:?}", cap);
            match cap.get(1).unwrap().as_str() {
                "SCHEMA" => Ok(LedgerObject::Schema(Schema::new(
                    cap.get(2).unwrap().as_str().to_string(),
                    cap.get(3).unwrap().as_str().to_string(),
                ))),
                "CLAIM_DEF" => Ok(LedgerObject::ClaimDef(ClaimDef::new(
                    cap.get(2).unwrap().as_str().to_string(),
                    cap.get(3).unwrap().as_str().to_string(),
                ))),

                _ => unimplemented!("Not yet completed"),
            }
        } else {
            println!("Requested DID does not match the W3C DID-core standard.");
            Err(RegexError::__Nonexhaustive)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Did {
    pub namespace: String,
    pub id: String,
    pub request_type: Type,
}

pub fn did_parse(did: &str) -> DidIndyResult<Did> {
    //TODO: change regex to exclude O,0,I,l
    let did_regex =
        Regex::new("did:indy:([a-zA-Z]+|[a-zA-Z]+:[a-zA-Z]+):([a-zA-Z0-9]{21,22})(/.*)?$")
            .expect("Error in the DID Regex Syntax");

    let captures = did_regex.captures(did.trim());
    return match captures {
        Some(cap) => {
            let did = Did {
                namespace: cap.get(1).unwrap().as_str().to_string(),
                id: cap.get(2).unwrap().as_str().to_string(),
                request_type: Type::Nym,
            };
            Ok(did)
        }
        None => {
            println!("Requested DID does not match the W3C DID-core standard.");
            Err(DidIndyError)
        }
    };
}

#[cfg(test)]
mod tests {
    mod did_syntax_tests {
        use crate::did::{did_parse, Did, Type};

        #[test]
        fn did_syntax_tests() {
            let _err = did_parse("did:indy:onlynamespace").unwrap_err();

            assert_eq!(
                did_parse("did:indy:idunion:BDrEcHc8Tb4Lb2VyQZWEDE").unwrap(),
                Did {
                    namespace: String::from("idunion"),
                    id: String::from("BDrEcHc8Tb4Lb2VyQZWEDE"),
                    request_type: Type::Nym
                }
            );

            assert_eq!(
                did_parse("did:indy:sovrin:staging:6cgbu8ZPoWTnR5Rv5JcSMB").unwrap(),
                Did {
                    namespace: String::from("sovrin:staging"),
                    id: String::from("6cgbu8ZPoWTnR5Rv5JcSMB"),
                    request_type: Type::Nym
                }
            );

            let _err =
                did_parse("did:indy:illegal:third:namespace:1111111111111111111111").unwrap_err();

            let _err = did_parse("did:indy:test:12345678901234567890").unwrap_err();
            let _err = did_parse("did:indy:test:12345678901234567890123").unwrap_err();
            //TODO: add Test to fail with namespace-identifer with O,0,I,l
        }
    }
}
