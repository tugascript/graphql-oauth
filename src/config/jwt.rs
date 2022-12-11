use std::{env, fs, path::Path};

#[derive(Clone)]
pub struct SingleJwt {
    pub secret: String,
    pub exp: i64,
}

#[derive(Clone)]
pub struct AccessJwt {
    pub private_key: String,
    pub public_key: String,
    pub exp: i64,
}

#[derive(Clone)]
pub struct Jwt {
    pub access: AccessJwt,
    pub reset: SingleJwt,
    pub confirmation: SingleJwt,
    pub refresh: SingleJwt,
    pub refresh_cookie: String,
    pub api_id: String,
}

impl Jwt {
    pub fn new() -> Self {
        let private_key = fs::read_to_string(Path::new("./keys/private.key")).unwrap();
        let public_key = fs::read_to_string(Path::new("./keys/public.key")).unwrap();
        let access_time = env::var("ACCESS_TIME").unwrap().parse::<i64>().unwrap();
        let reset_secret = env::var("RESET_SECRET").unwrap();
        let reset_time = env::var("RESET_TIME").unwrap().parse::<i64>().unwrap();
        let confirmation_secret = env::var("CONFIRMATION_SECRET").unwrap();
        let confirmation_time = env::var("CONFIRMATION_TIME")
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let refresh_secret = env::var("REFRESH_SECRET").unwrap();
        let refresh_time = env::var("REFRESH_TIME").unwrap().parse::<i64>().unwrap();

        Self {
            access: AccessJwt {
                private_key,
                public_key,
                exp: access_time,
            },
            reset: SingleJwt {
                secret: reset_secret,
                exp: reset_time,
            },
            confirmation: SingleJwt {
                secret: confirmation_secret,
                exp: confirmation_time,
            },
            refresh: SingleJwt {
                secret: refresh_secret,
                exp: refresh_time,
            },
            refresh_cookie: env::var("REFRESH_COOKIE").unwrap(),
            api_id: env::var("API_ID").unwrap(),
        }
    }
}
