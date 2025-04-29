use std::{
    fs::{self, File},
    error::Error,
    time::{Instant, SystemTime, Duration},
    thread
};

use sdl2::{
    event::Event, keyboard::Keycode, libc::sleep, mouse::MouseButton, pixels::Color, rect::Rect, render::{self, Canvas, Texture, TextureCreator}, ttf::{self, Font}, video::{self, Window, WindowContext}
};


const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT:u32 = 600;


#[derive(PartialEq)]
enum DisplayState {
    Standby,
    RunningTimer,
}


struct TextureData<'a> {
    font: Font<'a, 'static>,
    texture_creator: TextureCreator<WindowContext>,
}


struct VisualAsset<'a> {
    rect: Rect,
    background_color: Color,
    texture: Option<Texture<'a>>,
    text_color: Option<Color>,
}

impl<'a> VisualAsset<'a> {

    fn build(rect: Rect, background_color: Color, text: Option<String>, text_color: Option<Color>, data: Option<&'a TextureData>) -> Result<VisualAsset<'a>, Box<dyn Error>> {
        let texture = match (text, text_color) {
            (None, _) => None,
            (Some(_), None) => return Err("Text color is required when supplying text when buildng a VisualAsset".into()),
            (Some(text), Some(text_color)) => {
                match data {
                    None => return Err("TextureData needed to build VisualAsset with text".into()),
                    Some(data) => Some(create_text_texture(data, text, text_color)?),
                }
            },
        };

        let asset  = VisualAsset {
            rect, 
            background_color, 
            texture, 
            text_color,
        };
        Ok(asset)
    }

    fn update_texture(&mut self, data: &'a TextureData, text: String, text_color: Option<Color>) -> Result<(), Box<dyn Error>> {
        if self.text_color == None {
            match text_color {
                None => return Err("asset does not contain a text color. Supply one when calling update_texture".into()),
                Some(text_color) => self.text_color = Some(text_color),
            }
        }
        let texture: Texture<'a> = create_text_texture(data, text, self.text_color.unwrap())?;
        self.texture = Some(texture);

        Ok(())
    }
}


struct SleepPeriod {
    start: Option<Instant>,
    end: Option<Instant>,
    start_time: Option<SystemTime>
}


fn create_text_texture<'a>(data: &'a TextureData, text: String, color: Color) ->  Result<render::Texture<'a>, Box<dyn Error>>{
    let surface = data.font
    .render(text.as_str())
    .blended(color)
    .map_err(|e| e.to_string())?;
    let texture = data.texture_creator
    .create_texture_from_surface(&surface)
    .map_err(|e| e.to_string())?;
    Ok(texture)
}

fn is_leap_year(year: i64) -> bool {
    match year {
        _ if year % 400 == 0 => true,
        _ if year % 100 == 0 => false,
        _ if year % 4 == 0 => true,
        _ => false,
    }
}

fn year_from_seconds(total_secs: i64, year: i64) -> (i64, i64) {
    let days = match is_leap_year(year) {
        true => 366,
        false => 355
    };
 
    let secs_in_year = days * 24 * 60 * 60;
    if total_secs < secs_in_year {
        return (year, total_secs)
    } else {
        return year_from_seconds(total_secs - secs_in_year, year+1)
    }
}

enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    // fn increment(self) -> Month {
    //     match self {
    //         Month::January => Month::February,
    //         Month::February => Month::March,
    //         Month::March => Month::April,
    //         Month::April => Month::May,
    //         Month::May => Month::June,
    //         Month::June => Month::July,
    //         Month::July => Month::August,
    //         Month::August => Month::September,
    //         Month::September => Month::October,
    //         Month::October => Month::November,
    //         Month::November => Month::December,

    //     }
    // }
    fn increment(self) -> Month {
        match FromPrimitive::from_u8(self as u8 + 1) {
            Some(m2) => m2,
            None => FromPrimitive::from_u8(0),
        }
    }

}

fn month_from_seconds(total_secs: i64, leap_year: bool, month: Month) -> (i64,i64) {
    let days = match month {
        Month::January => 31,
        Month::February => {if leap_year {29} else {28}},
        Month::March => 31,
        Month::April => 30,
        Month::May => 31,
        Month::June => 30,
        Month::July => 31,
        Month::August => 31,
        Month::September => 30,
        Month::October => 31,
        Month::November => 30,
        Month::December => 31
    };

    let secs_in_month = days * 24 * 60 * 60;
    if total_secs <= secs_in_month {
        return (month as i64, total_secs)
    } else {
        return month_from_seconds(total_secs - secs_in_month, leap_year, month.increment())
    }
}

fn seconds_to_time(seconds: f64) -> String {
    let epoch_year = 1970;
    let tf = seconds as i64;
    let (year, remaining_secs): (i64, i64) = year_from_seconds(tf, epoch_year);
    let (month, remaining_secs): (i64, i64) = month_from_seconds(remaining_secs, is_leap_year(year), Month::January);
    let day = remaining_secs / (60 * 60 * 24);
    let total_seconds_today = remaining_secs % (60 * 60 * 24);
    let hours_today = total_seconds_today / (60 * 60);
    let minutes_today = (total_seconds_today / 60) % 60;
    let seconds_today = total_seconds_today % 60;

    let year: String = format!("{}", year);
    let month: String = if month < 10 {
        format!("0{}", month)
    } else {
        format!("{}", month)
    };
    let day: String = if day < 10 {
        format!("0{}", day)
    } else {
        format!("{}", day)
    };
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

    let current_time = format!("{}/{}/{} {}:{}:{}", year, month, day, hours_today, minutes_today, seconds_today);
    current_time 
}


fn get_current_time() -> Result<String, Box<dyn Error>> {
    let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let current_time = seconds_to_time(t.as_secs_f64());
    Ok(current_time)
}

fn write_sleep_period(db_path: &str, sleep_period: SleepPeriod) -> Result<(), Box<dyn Error>> {
    let file_exists: bool = std::fs::exists(db_path)?;
    let db: File = match file_exists {
        true => File::open(db_path)?,
        false => File::create(db_path)?,
    };

    let duration= sleep_period.end.unwrap().duration_since(sleep_period.start.unwrap());
    let time = seconds_to_time(duration.as_secs_f64());



    Ok(())
}


fn copy_visual_to_canvas<'a>(
    data: &'a TextureData<'a>, 
    canvas: &mut Canvas<Window>, 
    visual: &mut VisualAsset<'a>, 
    text: Option<String>) 
    -> Result<(), Box<dyn Error>> {
        if text.is_some() {
            visual.update_texture(data, text.unwrap(), None)?;
        }
        canvas.set_draw_color(visual.background_color);
        canvas.fill_rect(visual.rect)?;
        canvas.copy(visual.texture.as_ref().unwrap(), None, visual.rect)?;
        Ok(())
}


pub fn main() -> Result<(), Box<dyn Error>> {
    let db_path: &str = "sleep_log.txt";
    let font_path = "unifont.otf";
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("babyalarm", WINDOW_WIDTH, WINDOW_HEIGHT)
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

    let data: TextureData = TextureData { font, texture_creator };
    let mut state: DisplayState = DisplayState::Standby;

    let background_color: Color = Color::RGBA(50,50,200,255);
    let mut start_timer_button: VisualAsset = VisualAsset::build(
        Rect::new((WINDOW_WIDTH/2-100) as i32, (WINDOW_HEIGHT/2 - 50) as i32,200, 100),
        Color::RGBA(0,255,0,255),
        Some("Start".to_string()),
        Some(Color::RGBA(255,0,0,255)),
        Some(&data)
    )?;
    let mut resume_timer_button: VisualAsset = VisualAsset::build(
        Rect::new((WINDOW_WIDTH/2-300) as i32, (WINDOW_HEIGHT-150) as i32, 600, 100),
        Color::RGBA(0,0,0,255),
        Some("Resume previous timer".to_string()),
        Some(Color::RGBA(255,255,255,255)),
        Some(&data)
    )?;
    let mut stop_timer_button: VisualAsset = VisualAsset::build(
        Rect::new((WINDOW_WIDTH/2-100) as i32,(WINDOW_HEIGHT-150) as i32,200, 100),
        Color::RGBA(0,0,0,255),
        Some("Stop".to_string()),
        Some(Color::RGBA(255,255,255,255)),
        Some(&data)
    )?;



    let mut timer_visual: VisualAsset = VisualAsset::build(
        Rect::new(250,200,350,150),
        Color::RGBA(0,0,0,255),
        None,
        Some(Color::RGBA(255,255,255,255)),
        None
    )?;

    let mut time_started_visual: VisualAsset = VisualAsset::build(
        Rect::new(0,0,200,125),
        Color::RGBA(0,0,0,255),
        None,
        Some(Color::RGBA(255,255,255,255)),
        None
    )?;

    let mut clock_visual: VisualAsset = VisualAsset::build(
        Rect::new(700,0,100,50),
        Color::RGBA(0,0,0,255),
        None,
        Some(Color::RGBA(255,255,255,255)),
        None
    )?;

    // Timer
    let mut sleep_period: SleepPeriod = SleepPeriod { start: None, end: None, start_time: None};

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
                    match state {
                        DisplayState::Standby => {
                            // Start button click
                            let rect: Rect = start_timer_button.rect;
                            if x > rect.x && x < rect.x + rect.w
                                && y > rect.y && y < rect.y + rect.h {
                                println!("Timer started");
                                sleep_period.start_time = Some(SystemTime::now());
                                sleep_period.start = Some(Instant::now());
                                state = DisplayState::RunningTimer
                            }

                            // Resume button click
                            let rect = resume_timer_button.rect;
                            if x > rect.x && x < rect.x + rect.w && y > rect.y && y < rect.y + rect.w {
                                println!("Timer resumed");
                                sleep_period.end = None;
                                state = DisplayState::RunningTimer
                            }
                        },
                        DisplayState::RunningTimer => {
                            // Stop button click
                            if x > stop_timer_button.rect.x && x < stop_timer_button.rect.x + stop_timer_button.rect.w
                                && y > stop_timer_button.rect.y && y < stop_timer_button.rect.y + stop_timer_button.rect.h {
                                    println!("Timer stopped");
                                    sleep_period.end = Some(Instant::now());
                                    // write_sleep(&sleep_register, &sleep_period)?;
                                    state = DisplayState::Standby
                                }
                        }
                    }
                },
                _ => {}
            }
        }

        canvas.set_draw_color(background_color);
        canvas.clear();               

        match state {
            DisplayState::Standby => {
                // Draw start timer button
                copy_visual_to_canvas(&data, &mut canvas, &mut start_timer_button, None)?;
                
                // Draw button to resume the previously stopped running timer
                if sleep_period.end.is_some() {
                    copy_visual_to_canvas(&data, &mut canvas, &mut resume_timer_button, None)?;
                }
            },
            DisplayState::RunningTimer => {        
                // Calculate time passed and draw timer to screen
                let time_passed: Duration = sleep_period.start.unwrap().elapsed();
                let time_passed: String = seconds_to_time(time_passed.as_secs_f64()); 
                copy_visual_to_canvas(&data, &mut canvas, &mut timer_visual, Some(time_passed))?;
                
                // Draw the time the clock was started
                let duration_since_epoch= sleep_period.start_time.unwrap().duration_since(SystemTime::UNIX_EPOCH)?;
                let start_time = seconds_to_time(duration_since_epoch.as_secs_f64());
                copy_visual_to_canvas(&data, &mut canvas, &mut time_started_visual, Some(start_time))?;

                // Draw stop timer button
                copy_visual_to_canvas(&data, &mut canvas, &mut stop_timer_button, None)?;

            }
        }

        // Draw clock
        let current_time: String = get_current_time()?;
        copy_visual_to_canvas(&data, &mut canvas, &mut clock_visual, Some(current_time))?;

        // Render canvas
        canvas.present();


        thread::sleep(Duration::from_millis(100));

    }

    Ok(())
}
