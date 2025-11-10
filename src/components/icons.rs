use maud::Markup;

pub fn slash() -> Markup {
    maud::html! {
        svg viewBox="0 0 16 20" 
            width="16" 
            height="20" 
            fill="currentColor" 
            aria-hidden="true" 
            class="h-5 w-4 text-gray-300" { 
            path d="M5.697 4.34L8.98 16.532h1.327L7.025 4.341H5.697z" {}; } } }