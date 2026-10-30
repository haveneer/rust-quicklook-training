/// States: zero-sized types, they cost nothing at runtime.
#[derive(Default)]
pub struct Anonymous;
pub struct Authenticated;

/// The state is a type parameter: it exists only at compile time.
/// `Default` only where `State: Default`: no forged authenticated session.
#[derive(Default)]
pub struct Session<State> {
    user: String,
    _state: std::marker::PhantomData<State>,
}

impl Session<Anonymous> {
    /// Consumes the anonymous session; on failure, gives it back.
    pub fn login(self, user: &str, password: &str) -> Result<Session<Authenticated>, Self> {
        if !Self::check_credentials(user, password) {
            return Err(self);
        }
        let user = user.to_owned();
        Ok(Session {
            user,
            _state: std::marker::PhantomData,
        })
    }
    /// Stand-in: a real check would query an identity provider.
    fn check_credentials(user: &str, password: &str) -> bool {
        !user.is_empty() && !password.is_empty()
    }
}

/// These methods exist only once the session is authenticated.
impl Session<Authenticated> {
    pub fn read_logs(&self) -> String {
        format!("logs read by {}", self.user)
    }
    pub fn logout(self) -> Session<Anonymous> {
        Session::default()
    }
}

fn main() {
    let session = Session::<Anonymous>::default();
    // session.read_logs(); // error: no method `read_logs` for `Session<Anonymous>`
    let Ok(session) = session.login("alice", "correct horse") else {
        return;
    };
    println!("{}", session.read_logs());
    let _anonymous = session.logout();
    // session.read_logs(); // error: borrow of moved value: `session`
}

#[test]
fn test() {
    main()
}
