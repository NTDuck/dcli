use std::io::{self, Write};

// --- Boundaries ---

pub struct CounterRequest {
    pub change: i32, // Can be positive (increment) or negative (decrement)
}

pub struct CounterResponse {
    pub value: i32,
}

// --- ViewModel (Formatted Data for the View) ---

pub struct CounterViewModel {
    pub message: String,
}

// --- Boundaries (Input & Output) ---

pub trait CounterRequestBoundary {
    fn update_counter(&mut self, request: CounterRequest) -> CounterViewModel;
}

pub trait CounterResponseBoundary {
    fn format_response(&self, response: CounterResponse) -> CounterViewModel;
}

// --- Presenter (Formats Response into ViewModel) ---

pub struct CounterPresenter;

impl CounterResponseBoundary for CounterPresenter {
    fn format_response(&self, response: CounterResponse) -> CounterViewModel {
        CounterViewModel {
            message: format!("Counter: {}", response.value),
        }
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
    fn update_counter(&mut self, request: CounterRequest) -> CounterViewModel {
        self.value += request.change;
        let response = CounterResponse { value: self.value };
        self.presenter.format_response(response)
    }
}

// --- View (Handles Presentation) ---

pub struct CounterView;

impl CounterView {
    pub fn render(view_model: &CounterViewModel) {
        println!("{}", view_model.message);
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

    pub fn handle_input(&mut self, input: &str) -> CounterViewModel {
        match input.trim() {
            "inc" => self.interactor.update_counter(CounterRequest { change: 1 }),
            "dec" => self.interactor.update_counter(CounterRequest { change: -1 }),
            _ => CounterViewModel {
                message: "Unknown command. Use 'inc' to increment, 'dec' to decrement.".to_string(),
            },
        }
    }
}

// --- Main Console Loop ---

fn main() {
    let presenter = CounterPresenter;
    let interactor = CounterInteractor::new(presenter);
    let mut controller = CounterController::new(interactor);

    println!("Type 'inc' to increment, 'dec' to decrement. Type 'exit' to quit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Ensure prompt is displayed

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        let view_model = controller.handle_input(&input);
        CounterView::render(&view_model);
    }

    println!("Goodbye!");
}
