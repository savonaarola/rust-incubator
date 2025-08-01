use std::env;
use std::borrow::Cow;


    
    fn what_path() -> Cow<'static, str>{
        let mut args = env::args().skip(1);
        
        while let Some(arg) = args.next() {
            match &arg[..] {
                "--conf" => {
                    if let Some(arg_conf) = args.next() {
                        return Cow::Owned(arg_conf)
                    } else {panic!("Expected a value after --conf, but none was provided.")}
                },
                _ => ()

            }
            
        }

        if let Ok(val) = env::var("APP_CONF"){
            if !val.is_empty() {
                Cow::Owned(val)
            } else {
                Cow::Borrowed("/etc/app/app.conf")
            }
        } else {
            Cow::Borrowed("/etc/app/app.conf")
        }
    }
fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_path() {
        env::remove_var("APP_CONF");
        
        let path = what_path();
        assert_eq!(path, "/etc/app/app.conf");
    
        match path {
            Cow::Borrowed(_) => (),
            Cow::Owned(_) => panic!("Expected borrowed, got owned"),
        }
    }

    #[test]
    fn test_env_var_priority() {
        env::set_var("APP_CONF", "/custom/path/app.conf");
        
        let path = what_path();
        assert_eq!(path, "/custom/path/app.conf");
        
        match path {
            Cow::Borrowed(_) => panic!("Expected owned, got borrowed"),
            Cow::Owned(_) => (),
        }
        
        env::remove_var("APP_CONF");
    }
}