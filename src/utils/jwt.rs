use crate::{
    home::{Claims,HomeUser,Token},
};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use chrono::Utc;

/// secret key
const KEY: [u8; 16] = *include_bytes!("../secret.key");

pub fn get_token(user:HomeUser,exp:i64)->Token{
    let mut _token:String=String::new();
    if user.loginid.is_empty(){
        _token=String::from("");
    }else{
        let now=Utc::now().timestamp_nanos()/ 1_000_000_000; 
        let my_claims =Claims { 
            id:user.loginid,
            sub: "real_cpf@git.com".to_owned(),
            company: "real_cpf".to_owned(),
            px:user.rolex,
            py:user.roley,
            dept:user.deptid,
            iat:now,
            exp:now+exp,
        };
        _token = match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(&KEY)) {
            Ok(t) => t,
            Err(_) => String::from(""), 
        };
        
    }
    Token{
        token:_token
    }
}


pub fn from_token(token:String)->Claims{
    let _token=token.as_str();
    let mut res=String::new();
    // let validation = Validation { sub: Some("b@b.com".to_string()), ..Validation::default() };

    let token_data = match decode::<Claims>(_token, &DecodingKey::from_secret(&KEY), &Validation::default()) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
            ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
            _ => {
                
                panic!("Some other errors");
                
            },
        },
    };
    token_data.claims
}



