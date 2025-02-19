use std::{
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    task::{Context, Poll, Waker},
};

pub struct ManualFuture {
    // Indique si la future est prête
    ready: Arc<AtomicBool>,
    // Optionnel : permet de stocker le dernier waker fourni
    waker: Arc<Mutex<Option<Waker>>>,
}

impl ManualFuture {
    pub fn new() -> Self {
        ManualFuture {
            ready: Arc::new(AtomicBool::new(false)),
            waker: Arc::new(Mutex::new(None)),
        }
    }

    // Fonction pour signaler la complétion depuis un autre thread ou code externe
    pub fn signal(&self) {
        self.ready.store(true, Ordering::SeqCst);

        // Réveille le waker si présent
        if let Some(w) = self.waker.lock().unwrap().take() {
            w.wake();
        }
    }
}

impl Future for ManualFuture {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Vérifie si la future est prête
        if self.ready.load(Ordering::SeqCst) {
            // On peut retourner Poll::Ready
            Poll::Ready("Signal Received!")
        } else {
            // Sinon on enregistre le waker actuel pour un réveil futur
            let mut guard = self.waker.lock().unwrap();
            *guard = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

// Exemple d'utilisation dans un petit runtime
#[tokio::test] // On utilise l'executor de Tokio pour la démo
async fn main() {
    let my_future = ManualFuture::new();
    let handle = my_future.ready.clone();
    let signaller = my_future.waker.clone();

    let future_task = async move {
        println!("Future en attente...");
        let msg = my_future.await;
        println!("Future complétée : {}", msg);
    };

    // On lance la future sur la task async
    tokio::spawn(future_task);

    // Simulons un délai avant de signaler
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("On envoie le signal...");
    handle.store(true, Ordering::SeqCst);

    // On réveille le waker si besoin
    if let Some(waker) = signaller.lock().unwrap().take() {
        waker.wake();
    }

    // On attend un peu pour voir le résultat
    std::thread::sleep(std::time::Duration::from_secs(1));
}
