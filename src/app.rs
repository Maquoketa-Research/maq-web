use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use std::f64::consts::PI;
use std::time::Duration;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/maq-web.css"/>

        // sets the document title
        <Title text="Maquoketa Research - Innovation Through Data"/>
        
        // Add Google Fonts for Michroma
        <link rel="preconnect" href="https://fonts.googleapis.com"/>
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="true"/>
        <link href="https://fonts.googleapis.com/css2?family=Michroma:wght@400&display=swap" rel="stylesheet"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (is_loading, set_is_loading) = signal(true);
    let (dragon_complete, set_dragon_complete) = signal(false);
    
    // Provide the dragon completion signal to child components
    provide_context(set_dragon_complete);
    
    // Show content after dragon animation completes
    Effect::new(move |_| {
        if dragon_complete.get() {
            set_interval(
                move || {
                    set_is_loading.set(false);
                },
                Duration::from_millis(500), // Small delay for smooth transition
            );
        }
    });
    
    view! {
        <div class="page-container">
            <Show when=move || is_loading.get()>
                <div class="loading-screen">
                    <div class="dragon-intro">
                        <DragonFractal/>
                        <div class="loading-text">
                            <h1>"Maquoketa Research"</h1>
                            <p>"Loading Innovation..."</p>
                        </div>
                    </div>
                </div>
            </Show>
            
            <Show when=move || !is_loading.get()>
                <div class="main-content">
                    <Header/>
                    <main class="page-content">
                        <HeroSection/>
                        <ServicesSection/>
                        <TeamSection/>
                        <ContactSection/>
                    </main>
                    <Footer/>
                </div>
            </Show>
        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="header">
            <nav class="nav">
                <div class="nav-brand">
                    <h1 class="logo">"Maquoketa Research"</h1>
                </div>
                <ul class="nav-links">
                    <li><a href="#about">"About"</a></li>
                    <li><a href="#services">"Services"</a></li>
                    <li><a href="#team">"Team"</a></li>
                    <li><a href="#contact">"Contact"</a></li>
                </ul>
            </nav>
        </header>
    }
}

// Generate the final dragon curve (8 iterations - one stage further than animation) for the logo
fn generate_final_dragon_logo() -> String {
    let mut points = vec![Point::new(0.0, 0.0), Point::new(1.0, 0.0)];
    
    // Generate 8 iterations (one more than the animation's 7)
    for _ in 0..11 {
        points = generate_next_iteration(&points);
    }
    
    // Calculate bounds for scaling
    let mut min_x = points[0].x;
    let mut max_x = points[0].x;
    let mut min_y = points[0].y;
    let mut max_y = points[0].y;
    
    for point in &points {
        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);
    }
    
    let width = max_x - min_x;
    let height = max_y - min_y;
    let padding = 100.0; // More padding to prevent cutoff
    let scale = (600.0 - padding) / width.max(height).max(0.1); // Much larger scale
    
    let offset_x = 400.0 - (min_x + max_x) * scale / 2.0; // Center in larger viewBox
    let offset_y = 300.0 - (min_y + max_y) * scale / 2.0; // Center in larger viewBox
    
    let mut path = format!("M {} {}", 
        points[0].x * scale + offset_x, 
        points[0].y * scale + offset_y
    );
    
    for point in points.iter().skip(1) {
        path.push_str(&format!(" L {} {}", 
            point.x * scale + offset_x, 
            point.y * scale + offset_y
        ));
    }
    
    path
}

#[component]
fn DragonLogo() -> impl IntoView {
    let logo_path = generate_final_dragon_logo();
    
    view! {
        <div class="dragon-logo">
            <svg width="800" height="600" viewBox="0 0 800 600">
                <path
                    d=logo_path
                    stroke="rgba(255, 255, 255, 0.9)"
                    stroke-width="3"
                    fill="none"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                />
            </svg>
        </div>
    }
}

#[component]
fn HeroSection() -> impl IntoView {
    view! {
        <section class="hero">
            <div class="hero-content">
                <div class="hero-logo">
                    <DragonLogo/>
                </div>
                <p class="hero-subtitle">"There are bears"</p>
                <div class="hero-buttons">
                    <a href="#contact" class="btn btn-primary">"Get Started"</a>
                    <a href="#about" class="btn btn-secondary">"Learn More"</a>
                </div>
            </div>
        </section>
    }
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    fn rotate_around(&self, center: Point, angle: f64) -> Point {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let dx = self.x - center.x;
        let dy = self.y - center.y;
        
        Point {
            x: center.x + dx * cos_a - dy * sin_a,
            y: center.y + dx * sin_a + dy * cos_a,
        }
    }
}

#[derive(Clone, Debug)]
struct DragonState {
    original_points: Vec<Point>,
    iteration: usize,
    animation_progress: f64, // 0.0 to 1.0 for smooth animation
    is_animating: bool,
}

// Generate the next iteration of dragon curve by duplication and rotation
fn generate_next_iteration(current_points: &[Point]) -> Vec<Point> {
    if current_points.len() < 2 {
        return current_points.to_vec();
    }
    
    let mut new_points = current_points.to_vec();
    let end_point = current_points[current_points.len() - 1];
    
    // Create rotated copy (90 degrees counterclockwise around endpoint)
    for i in (0..current_points.len() - 1).rev() {
        let rotated = current_points[i].rotate_around(end_point, PI / 2.0);
        new_points.push(rotated);
    }
    
    new_points
}

// Get the rotated copy points for animation
fn get_rotated_copy(original_points: &[Point], rotation_angle: f64) -> Vec<Point> {
    if original_points.len() < 2 {
        return vec![];
    }
    
    let end_point = original_points[original_points.len() - 1];
    let mut rotated_points = Vec::new();
    
    // Create rotated copy (in reverse order) around the endpoint
    // The rotation should be 90 degrees counterclockwise (PI/2)
    for i in (0..original_points.len() - 1).rev() {
        let rotated = original_points[i].rotate_around(end_point, rotation_angle);
        rotated_points.push(rotated);
    }
    
    rotated_points
}

// Get all points for the current animation state (original + partial rotated copy)
fn get_current_animation_points(original_points: &[Point], rotation_angle: f64) -> Vec<Point> {
    if original_points.len() < 2 {
        return original_points.to_vec();
    }
    
    let mut all_points = original_points.to_vec();
    let rotated_points = get_rotated_copy(original_points, rotation_angle);
    all_points.extend(rotated_points);
    
    all_points
}

#[component]
fn DragonFractal() -> impl IntoView {
    let (dragon_state, set_dragon_state) = signal(DragonState {
        original_points: vec![Point::new(0.0, 0.0), Point::new(1.0, 0.0)],
        iteration: 0,
        animation_progress: 0.0,
        is_animating: false,
    });
    let max_iterations = 10;
    let (_cycle_count, set_cycle_count) = signal(0);
    
    // Get completion signal from context
    let completion_signal = use_context::<WriteSignal<bool>>();
    
    // Start the animation
    Effect::new(move |_| {
        set_dragon_state.update(|state| {
            state.is_animating = true;
        });
    });
    
    // Animation loop
    Effect::new(move |_| {
        set_interval(
            move || {
                set_dragon_state.update(|state| {
                    if !state.is_animating {
                        return;
                    }
                    
                    if state.animation_progress < 1.0 {
                        // Continue current iteration animation (rotation)
                        state.animation_progress += 0.04; // Twice as fast
                        if state.animation_progress >= 1.0 {
                            state.animation_progress = 1.0;
                        }
                    } else {
                        // Animation complete, merge the curves and move to next iteration
                        if state.iteration < max_iterations {
                            state.original_points = generate_next_iteration(&state.original_points);
                            state.iteration += 1;
                            state.animation_progress = 0.0;
                        } else {
                            // Reached final iteration - merge the final rotation and stop
                            state.original_points = generate_next_iteration(&state.original_points);
                            state.is_animating = false;
                            state.animation_progress = 1.0; // Ensure we show the final complete state
                            
                            set_cycle_count.update(|count| *count += 1);
                            
                            // Signal completion
                            if let Some(signal) = completion_signal {
                                signal.set(true);
                            }
                        }
                    }
                });
            },
            Duration::from_millis(25), // 25ms intervals for twice the speed
        );
    });
    
    let create_path = move |points: &[Point]| -> String {
        if points.is_empty() {
            return String::new();
        }
        
        // Calculate bounds for scaling
        let mut min_x = points[0].x;
        let mut max_x = points[0].x;
        let mut min_y = points[0].y;
        let mut max_y = points[0].y;
        
        for point in points {
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
        }
        
        let width = max_x - min_x;
        let height = max_y - min_y;
        let padding = 50.0;
        let scale = (350.0 - padding) / width.max(height).max(0.1);
        
        let offset_x = 200.0 - (min_x + max_x) * scale / 2.0;
        let offset_y = 150.0 - (min_y + max_y) * scale / 2.0;
        
        let mut path = format!("M {} {}", 
            points[0].x * scale + offset_x, 
            points[0].y * scale + offset_y
        );
        
        for point in points.iter().skip(1) {
            path.push_str(&format!(" L {} {}", 
                point.x * scale + offset_x, 
                point.y * scale + offset_y
            ));
        }
        
        path
    };
    
    // Get current state data for rendering
    // Get the complete curve path (original + rotated copy during animation)
    let complete_dragon_path = move || {
        let state = dragon_state.get();
        
        if !state.is_animating {
            // Show only the original points when not animating (final merged state)
            create_path(&state.original_points)
        } else {
            // Show original + rotated copy during animation
            let rotation_angle = state.animation_progress * PI / 2.0; // 0 to 90 degrees
            let all_points = get_current_animation_points(&state.original_points, rotation_angle);
            create_path(&all_points)
        }
    };
    
    view! {
        <div class="dragon-fractal-container">
            <svg width="400" height="300" viewBox="0 0 400 300">
                // Single unified dragon curve (white)
                <path
                    d=complete_dragon_path
                    stroke="rgba(255, 255, 255, 0.9)"
                    stroke-width="2"
                    fill="none"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="dragon-unified"
                />
            </svg>
        </div>
    }
}

#[component]
fn AboutSection() -> impl IntoView {
    view! {
        <section id="about" class="about">
            <div class="container">
                <h2 class="section-title">"About Maquoketa Research"</h2>
                <p class="section-description">
                    "Founded on the principle that data drives innovation, Maquoketa Research specializes in cutting-edge research methodologies and data analysis. We bridge the gap between complex research questions and practical solutions."
                </p>
            </div>
        </section>
    }
}

#[component]
fn ServicesSection() -> impl IntoView {
    view! {
        <section id="services" class="services">
            <div class="container">
                <h2 class="section-title">"Our Products"</h2>
                <div class="services-grid">
                    <div class="service-card">
                        <h3>"Control Freak"</h3>
                        <p>"Fully automated motor control and system identification powered by proprietary data-driven algorithms"</p>
                    </div>
                    <div class="service-card">
                        <h3>"VLA System"</h3>
                        <p>"Coming Soon"</p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn TeamSection() -> impl IntoView {
    view! {
        <section id="team" class="team">
            <div class="container">
                <h2 class="section-title">"Our Team"</h2>
                <div class="team-grid">
                    <div class="team-member">
                        <div class="member-avatar evan-avatar"></div>
                        <h3>"Evan Yeager"</h3>
                        <p class="member-role">"Head of Research"</p>
                        <p class="member-bio">"Evan, Co-Founder and CRO, started programming at 8 years old, placed in a national math competition at 16, and began doing quality research at 17. \
                        Before founding Maquoketa he studied Electrical Engineering and Computer Science at Berkeley and worked briefly in the financial industry building derivatives trading software."</p>
                    </div>
                    <div class="team-member">
                        <div class="member-avatar aidan-avatar"></div>
                        <h3>"Aidan Williams"</h3>
                        <p class="member-role">"Head of Technology"</p>
                        <p class="member-bio">"Massive Gooner"</p>
                    </div>
                    <div class="team-member">
                        <div class="member-avatar dave-avatar"></div>
                        <h3>"Dave Muchow"</h3>
                        <p class="member-role">"Head of Operations"</p>
                        <p class="member-bio">"Even bigger gooner"</p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn ContactSection() -> impl IntoView {
    view! {
        <section id="contact" class="contact">
            <div class="container">
                <h2 class="section-title">"Get In Touch"</h2>
                <div class="contact-content">
                    <div class="contact-info">
                        <h3>"Contact Information"</h3>
                        <p>"📧 yeager@berkeley.edu"</p>
                        <p>"📍 780 Bonnie Ln, Elk Grove Village, IL 60007"</p>
                    </div>
                    <div class="contact-form">
                        <h3>"Send us a message"</h3>
                        <form>
                            <input type="text" placeholder="Your Name" required/>
                            <input type="email" placeholder="Your Email" required/>
                            <textarea placeholder="Your Message" rows="4" required></textarea>
                            <button type="submit" class="btn btn-primary">"Send Message"</button>
                        </form>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="container">
                <p>"© 2025 Maquoketa Research. All rights reserved."</p>
                <p>""</p>
            </div>
        </footer>
    }
}
