use std::io::{self, Write};

// --- Boundaries ---

pub struct CounterRequest {
    pub increment: i32,
}

pub struct CounterResponse {
    pub value: i32,
}

pub trait CounterRequestBoundary {
    fn increment(&mut self, request: CounterRequest);
}

pub trait CounterResponseBoundary {
    fn update_counter(&mut self, response: CounterResponse);
}

// --- Presenter (Implements ResponseBoundary) ---

pub struct CounterPresenter;

impl CounterResponseBoundary for CounterPresenter {
    fn update_counter(&mut self, response: CounterResponse) {
        println!("Counter updated: {}", response.value);
    }
}

// --- Interactor (Implements RequestBoundary & Uses ResponseBoundary) ---

pub struct CounterInteractor<R: CounterResponseBoundary> {
    presenter: R,
    value: i32,
}

impl<R: CounterResponseBoundary> CounterInteractor<R> {
    pub fn new(presenter: R) -> Self {
        Self { presenter, value: 0 }
    }
}

impl<R: CounterResponseBoundary> CounterRequestBoundary for CounterInteractor<R> {
    fn increment(&mut self, request: CounterRequest) {
        self.value += request.increment;
        self.presenter.update_counter(CounterResponse { value: self.value });
    }
}

// --- Controller (Uses RequestBoundary) ---

pub struct CounterController<I: CounterRequestBoundary> {
    interactor: I,
}

impl<I: CounterRequestBoundary> CounterController<I> {
    pub fn new(interactor: I) -> Self {
        Self { interactor }
    }

    pub fn on_input(&mut self, input: &str) {
        if input.trim() == "inc" {
            let request = CounterRequest { increment: 1 };
            self.interactor.increment(request);
        }
    }
}

// --- Main Console Loop ---

fn main() {
    let presenter = CounterPresenter;
    let interactor = CounterInteractor::new(presenter);
    let mut controller = CounterController::new(interactor);

    println!("Type 'inc' to increment the counter. Type 'exit' to quit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Ensure prompt is displayed

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        controller.on_input(&input);
    }

    println!("Goodbye!");
}
