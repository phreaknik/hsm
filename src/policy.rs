use crate::signing::{SigRequest, SigType};
use microlisp::{Environment, Expression};
use serde::{Deserialize, Serialize};

pub type PolicyUID = blake3::Hash;

#[derive(Deserialize, Serialize)]
pub struct Policy {
    sigtype: SigType,
    script: String,
}

impl Policy {
    pub fn new(sigtype: SigType, script: String) -> Policy {
        Policy { sigtype, script }
    }

    pub fn identifier(&self) -> PolicyUID {
        blake3::hash(&bincode::serialize(self).unwrap())
    }

    pub fn approves(&self, request: &SigRequest) -> bool {
        if self.sigtype == request.sigtype() {
            let mut env = Environment::new();
            env.load_default_builtins().unwrap();
            let script = self.script.parse().unwrap();
            match request {
                SigRequest::Psbt(_) => Ok(Expression::Bool(true)) == env.eval(script),
                SigRequest::Message(_) => Ok(Expression::Bool(true)) == env.eval(script),
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
