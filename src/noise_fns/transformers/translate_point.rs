use math::{Point2, Point3, Point4};
use noise_fns::NoiseFn;

/// Noise function that moves the coordinates of the input value before
/// returning the output value from the source function.
///
/// The get() method moves the coordinates of the input value by a translation
/// amount before returning the output value from the source function.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TranslatePoint<Source> {
    /// Source function that outputs a value
    pub source: Source,

    /// Translation amount applied to the _x_ coordinate of the input value.
    /// The default translation amount is set to 0.0.
    pub x_translation: f64,

    /// Translation amount applied to the _y_ coordinate of the input value.
    /// The default translation amount is set to 0.0.
    pub y_translation: f64,

    /// Translation amount applied to the _z_ coordinate of the input value.
    /// The default translation amount is set to 0.0.
    pub z_translation: f64,

    /// Translation amount applied to the _u_ coordinate of the input value.
    /// The default translation amount is set to 0.0.
    pub u_translation: f64,
}

impl<'a, Source> TranslatePoint<Source> {
    pub fn new(source: Source) -> Self {
        TranslatePoint {
            source,
            x_translation: 0.0,
            y_translation: 0.0,
            z_translation: 0.0,
            u_translation: 0.0,
        }
    }

    /// Sets the scaling factor to apply to the _x_ coordinate of the input
    /// value.
    pub fn set_x_translation(self, x_translation: f64) -> Self {
        TranslatePoint {
            x_translation,
            ..self
        }
    }

    /// Sets the scaling factor to apply to the _y_ coordinate of the input
    /// value.
    pub fn set_y_translation(self, y_translation: f64) -> Self {
        TranslatePoint {
            y_translation,
            ..self
        }
    }

    /// Sets the scaling factor to apply to the _z_ coordinate of the input
    /// value.
    pub fn set_z_translation(self, z_translation: f64) -> Self {
        TranslatePoint {
            z_translation,
            ..self
        }
    }

    /// Sets the scaling factor to apply to the _u_ coordinate of the input
    /// value.
    pub fn set_u_translation(self, u_translation: f64) -> Self {
        TranslatePoint {
            u_translation,
            ..self
        }
    }

    /// Sets the translation amount to apply to all coordinates of the input value.
    pub fn set_translation(self, scale: f64) -> Self {
        TranslatePoint {
            x_translation: scale,
            y_translation: scale,
            z_translation: scale,
            u_translation: scale,
            ..self
        }
    }

    /// Sets the individual translation amounts to apply to each coordinate of
    /// the input value.
    pub fn set_all_translations(
        self,
        x_translation: f64,
        y_translation: f64,
        z_translation: f64,
        u_translation: f64,
    ) -> Self {
        TranslatePoint {
            x_translation,
            y_translation,
            z_translation,
            u_translation,
            ..self
        }
    }
}

impl<Source> NoiseFn<Point2<f64>> for TranslatePoint<Source>
where
    Source: NoiseFn<Point2<f64>>,
{
    fn get(&self, point: Point2<f64>) -> f64 {
        self.source
            .get([point[0] + self.x_translation, point[1] + self.y_translation])
    }
}

impl<Source: NoiseFn<Point3<f64>>> NoiseFn<Point3<f64>> for TranslatePoint<Source>
where
    Source: ,
{
    fn get(&self, point: Point3<f64>) -> f64 {
        self.source.get([
            point[0] + self.x_translation,
            point[1] + self.y_translation,
            point[2] + self.z_translation,
        ])
    }
}

impl<Source: NoiseFn<Point4<f64>>> NoiseFn<Point4<f64>> for TranslatePoint<Source> {
    fn get(&self, point: Point4<f64>) -> f64 {
        self.source.get([
            point[0] + self.x_translation,
            point[1] + self.y_translation,
            point[2] + self.z_translation,
            point[3] + self.u_translation,
        ])
    }
}
