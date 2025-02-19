use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

/// Structure partagée entre le Future et le thread qui "dort".
struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

/// Notre Future personnalisé qui "attend" un certain temps.
pub struct Delay {
    shared_state: Arc<Mutex<SharedState>>,
}

impl Delay {
    /// Crée un nouveau Delay qui sera complété après `dur`.
    pub fn new(dur: Duration) -> Self {
        // Prépare l'état partagé
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // Clone pour le thread
        let thread_shared_state = shared_state.clone();

        // Lance un thread en arrière-plan
        thread::spawn(move || {
            // On dort le temps spécifié
            thread::sleep(dur);
            // On indique que l'opération est terminée
            let mut state = thread_shared_state.lock().unwrap();
            state.completed = true;
            // On réveille la tâche asynchrone qui est en attente (si elle existe)
            if let Some(waker) = state.waker.take() {
                waker.wake();
            }
        });

        Delay { shared_state }
    }
}

// Implémentation du trait Future pour Delay
impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // On récupère l'état partagé
        let mut state = self.shared_state.lock().unwrap();
        if state.completed {
            // Si l'opération est terminée, on renvoie un Poll::Ready
            Poll::Ready(())
        } else {
            // Sinon on enregistre le waker pour qu'on puisse le réveiller plus tard
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

use futures::executor::block_on;

#[test]
fn main() {
    println!("Début de la future Delay...");
    // On bloque le thread principal jusqu'à la fin du Future
    block_on(Delay::new(Duration::from_secs(2)));
    println!("Delay complété !");
}