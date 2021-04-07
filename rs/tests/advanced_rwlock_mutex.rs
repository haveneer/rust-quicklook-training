use std::sync::{RwLock, Mutex};

#[test]
fn test_rwlock() {
    let shared_data = RwLock::new(5);

    // many reader locks can be held at once
    {
        let r1 = shared_data.read().unwrap();
        let r2 = shared_data.read().unwrap(); // simulate multiple read lock: OK
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // read locks are dropped at this point

    // only one write lock may be held, however
    {
        let mut w = shared_data.write().unwrap();
        // let mut w2 = lock.write().unwrap(); // multiple write lock will panic
        *w += 1;
        assert_eq!(*w, 6);
    } // write lock is dropped here
}

#[test]
fn test_mutex() {
    let shared_data = Mutex::new(5);

    // one lock can be held at once
    {
        let r = shared_data.lock().unwrap();
        // let r2 = shared_data.lock().unwrap(); // infinite lock
        assert_eq!(*r, 5);
    } // lock is dropped at this point

    // one lock can be held at once
    {
        let mut w = shared_data.lock().unwrap();
        // let mut w2 = shared_data.lock().unwrap(); // infinite lock
        *w += 1;
        assert_eq!(*w, 6);
    } // write lock is dropped here
    
    
}