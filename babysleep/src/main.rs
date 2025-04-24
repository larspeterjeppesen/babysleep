use std::{
    error::Error,
    time::{Instant, SystemTime, Duration},
    thread
};

use sdl2::{
    event::Event, keyboard::Keycode, libc::sleep, mouse::MouseButton, pixels::Color, rect::Rect, render::{self, Canvas, Texture, TextureCreator}, ttf::{self, Font}, video::{self, Window, WindowContext}
};

#[derive(PartialEq)]
enum DisplayState {
    Standby,
    RunningTimer,
}


struct AppData<'a> {
    font: Font<'a, 'static>,
    texture_creator: TextureCreator<WindowContext>,
    // canvas: Canvas<Window>
    // clock_texture: Texture<'a>
}

struct Button<'a> 
// where F: FnMut() -> ()
{
    rect: Rect,
    texture: Option<Texture<'a>>,
    background_color: Color,
    // button_press: F,
    // button_press: impl FnMut()
}

// struct Timer {
//     start: Option<SystemTime>,
//     end: Option<SystemTime>,
//     rect: Rect, 
// }

struct SleepPeriod {
    start: Option<Instant>,
    end: Option<Instant>,
    start_time: Option<SystemTime>
}


fn create_text_texture<'a>(data: &'a AppData, text: String, color: Color) ->  Result<render::Texture<'a>, Box<dyn Error>>{
    let surface = data.font
    .render(text.as_str())
    .blended(color)
    .map_err(|e| e.to_string())?;
    let texture = data.texture_creator
    .create_texture_from_surface(&surface)
    .map_err(|e| e.to_string())?;
    Ok(texture)
}

fn seconds_to_time(seconds: f64) -> String {
    let tf = seconds as i64;
    let total_seconds_today = tf % (60 * 60 * 24);
    let hours_today = total_seconds_today / (60 * 60);
    let minutes_today = (total_seconds_today / 60) % 60;
    let seconds_today = total_seconds_today % 60;

    let hours_today: String = if hours_today < 10 {
        format!("0{}", hours_today)
    } else {
        format!("{}", hours_today)
    };
    let minutes_today: String = if minutes_today < 10 {
        format!("0{}", minutes_today)
    } else {
        format!("{}", minutes_today)
    };
    let seconds_today: String = if seconds_today < 10 {
        format!("0{}", seconds_today)
    } else {
        format!("{}", seconds_today)
    };

    let current_time = format!("{}:{}:{}", hours_today, minutes_today, seconds_today);
    current_time 
}


fn get_current_time() -> Result<String, Box<dyn Error>> {
    let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let current_time = seconds_to_time(t.as_secs_f64());
    Ok(current_time)
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let font_path = "unifont.otf";
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("babyalarm", 800, 600)
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;



    // Create assets
    let texture_creator = canvas.texture_creator();

    let ttf_context = ttf::init().map_err(|e| e.to_string())?;
    let mut font: Font<'_, 'static> = ttf_context.load_font(font_path, 128)?;
    font.set_style(ttf::FontStyle::BOLD);

    let data: AppData = AppData { font, texture_creator };

    let mut state: DisplayState = DisplayState::Standby;

    // Start button
    let background_color: Color = Color::RGBA(0,255,0,0);
    let text_color: Color = Color::RGBA(255,0,0,255);
    let start_timer_texture = create_text_texture(&data, "Start".to_string(), text_color)?;
    let start_button_rect = Rect::new(100, 100, 200, 100);

    // Running timer
    let running_timer_rect= Rect::new(250, 200, 350, 150);
    let start_time_rect = Rect::new(100, 100, 200, 125);


    // Clock
    let clock_rect = Rect::new(700, 0, 100, 50);

    // Timer
    let mut sleep_period: SleepPeriod = SleepPeriod { start: None, end: None, start_time: None};


    // let button_press = || {
    //     state = DisplayState::RunTimer;
    //     if timer.end == None {
    //         timer.start = Some(SystemTime::now());
    //     } else {
    //         timer
    //     }
    // };

    let start_timer_button: Button = Button { 
        rect: start_button_rect, 
        texture: Some(start_timer_texture), 
        background_color,
        // button_press
    };




    canvas.set_draw_color(Color::RGBA(0,0,255,0));
    canvas.clear();

    canvas.set_draw_color(start_timer_button.background_color);
    canvas.fill_rect(start_button_rect)?;
    canvas.copy(&start_timer_button.texture.unwrap(), None, start_timer_button.rect)?;
    canvas.present();




    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                Event::MouseButtonDown { 
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    if state == DisplayState::Standby {
                        // Start button click
                        if x > start_timer_button.rect.x && x < start_timer_button.rect.x + start_timer_button.rect.w
                            && y > start_timer_button.rect.y && y < start_timer_button.rect.y + start_timer_button.rect.h {
                            println!("Timer started");
                            sleep_period.start_time = Some(SystemTime::now());
                            sleep_period.start = Some(Instant::now());
                            state = DisplayState::RunningTimer
                        }
                    }
                },
                _ => {}
            }
        }

        match state {
            DisplayState::Standby => (),
            DisplayState::RunningTimer => {
                // Calculate time passed and draw timer to screen
                let time_passed: Duration = sleep_period.start.unwrap().elapsed();
                let time_passed: String = seconds_to_time(time_passed.as_secs_f64()); 
                let text_color: Color = Color::RGBA(255,0,0,255);
                let running_timer_texture= create_text_texture(&data, time_passed, text_color)?;
                
                canvas.set_draw_color(background_color);
                canvas.clear();
                
                let rect_color = Color::RGBA(0, 0, 0, 255);
                canvas.set_draw_color(rect_color);
                canvas.fill_rect(running_timer_rect)?;
                canvas.copy(&running_timer_texture, None, running_timer_rect)?;

                // Draw the time the clock was started
                let duration_since_epoch= sleep_period.start_time.unwrap().duration_since(SystemTime::UNIX_EPOCH)?;
                let text_color: Color = Color::RGBA(255,255,255,255);
                let start_time = seconds_to_time(duration_since_epoch.as_secs_f64());
                let start_time_texture = create_text_texture(&data, start_time, text_color)?;
                let rect_color = Color::RGBA(0,0,0,255); 
                canvas.set_draw_color(rect_color);
                canvas.fill_rect(start_time_rect)?;
                canvas.copy(&start_time_texture, None, start_time_rect)?;


            }
        }

        // Draw clock
        let current_time: String= get_current_time()?;
        let text_color: Color = Color::RGBA(255,255,255,255);
        let clock_texture = create_text_texture(&data, current_time, text_color)?;
        let rect_color = Color::RGBA(0,0,0,0);
        canvas.set_draw_color(rect_color);
        canvas.fill_rect(clock_rect)?;
        canvas.copy(&clock_texture, None, clock_rect)?;

        canvas.present();
        thread::sleep(Duration::from_millis(100));



        // canvas.set_draw_color(color);
        // // canvas.fill_rect(rect)?;

    }

    Ok(())
}
