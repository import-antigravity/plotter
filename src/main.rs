use iced::application::Update;
use iced::mouse::Cursor;
use iced::widget::canvas;
use iced::widget::canvas::Geometry;
use iced::{Rectangle, Renderer, Theme};

#[derive(Debug, Clone)]
struct Range<T = f32> {
    min: T,
    max: T,
    increment: T,
}

impl Range {
    pub fn new(min: f32, max: f32, increment: f32) -> Self {
        Range {
            min,
            max,
            increment,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=f32> + use<'_> {
        let start = (self.min / self.increment).trunc() * self.increment;
        let end = (self.max / self.increment).trunc() * self.increment;
        let n_steps = ((end - start) / self.increment).trunc() as i32;
        (0..=n_steps).into_iter().map(move |step| {
            let state = step as f32 * self.increment;
            start + state
        })
    }
}

#[derive(Debug, Clone)]
enum Axis {
    X,
    Y,
}

impl Axis {
    fn range(&self, min: f32, max: f32, increment: f32) -> Range
    {

        Range::new(min, max, increment)
    }
}

#[derive(Debug, Clone)]
struct Axes {
    x_range: Range,
    y_range: Range,
}

struct Figure {
    axes: Axes,
}

impl canvas::Program<Message> for Figure {
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        // draw axes
        todo!()
    }
}

#[derive(Default, Debug)]
struct Plotter {}

#[derive(Debug, Clone)]
enum Message {}

impl Plotter {
    fn update(&mut self, message: Message) {
        todo!()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        todo!()
    }
}

fn main() -> iced::Result {
    iced::application("test plotter", Plotter::update, Plotter::view).run()
}

mod test {
    use crate::Range;

    #[test]
    fn test_null_range() {
        let range = Range::new(1.0, -1.0, 1.0);
        assert!(range.iter().next().is_none());
    }

    #[test]
    fn test_range_one_on_origin() {
        let range = Range::new(0.0, 0.0, 1.0);
        assert_eq!(range.iter().collect::<Vec<_>>(), vec![0.0]);
    }

    #[test]
    fn test_range_one_across_origin() {
        let range = Range::new(-0.5, 0.5, 1.0);
        assert_eq!(range.iter().collect::<Vec<_>>(), vec![0.0]);
    }
}
