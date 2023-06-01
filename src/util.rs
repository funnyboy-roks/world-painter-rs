use color_art::Color;
use image::Rgb;

pub trait MinMax<T> {
    fn min_max(self) -> (T, T);
}

impl<T, I> MinMax<T> for I
where
    I: Iterator<Item = T>,
    T: Ord + Copy,
{
    fn min_max(self) -> (T, T) {
        let (min, max) = self.fold::<(Option<T>, Option<T>), _>((None, None), |(min, max), c| {
            let mut out_min = min;
            if let Some(min) = min {
                if min > c {
                    out_min = Some(c)
                }
            } else {
                out_min = Some(c);
            }
            let mut out_max = max;
            if let Some(max) = max {
                if max < c {
                    out_max = Some(c)
                }
            } else {
                out_max = Some(c);
            }

            (out_min, out_max)
        });
        (min.unwrap(), max.unwrap())
    }
}

pub fn color_to_rgb(col: Color) -> Rgb<u8> {
    Rgb([col.red(), col.green(), col.blue()])
}

pub fn map_num(n: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
    ((n - start1) / (stop1 - start1)) * (stop2 - start2) + start2
}
