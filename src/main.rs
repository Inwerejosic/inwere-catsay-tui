use cursive::views::TextView;
use cursive::event::Key;

fn main() {
    let mut siv = cursive::default();
    
    let cat_text = "Meow!
    \\
     \\
       /\\_/\\
      ( o.o )
      =( I )=";

      // Declaring the app layout.
    siv.add_layer(TextView::new(cat_text));

    siv.add_global_callback(Key::Esc, |s| s.quit());
    
    siv.run(); //Starting the event loop.
}
