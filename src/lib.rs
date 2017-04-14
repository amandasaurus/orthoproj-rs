//! Create orthographic projection images in Rust

/// An orthographic image
/// 
/// Images are square, with the globe in the middle. Create one with a size of 500x500, where
/// all pixels are set to `0`, centred on Rome:
///
/// # Examples
///
///```
///use orthoproj::OrthoProj;
///let mut image = OrthoProj::new(500, 41.89889, 12.47337, 0);
///```
///
/// We can then set pixels on this image, based on their lat/lon
///
///```
///# use orthoproj::OrthoProj;
///# let mut image = OrthoProj::new(500, 41.89889, 12.47337, 0);
///image.set(51.50791, -0.12786, 1);
///```
///
/// You can then loop over all the pixels, getting the current value.
///
pub struct OrthoProj<T: Clone> {
    _data: Vec<T>,
    _lat: f32,
    _lon: f32,
    _size: u32,
}

impl<T: Clone> OrthoProj<T> {
    /// Create a new orthographic projection with width & height of `size`, centred on `lat` and
    /// `lon`. `default` is the default value
    pub fn new(size: u32, lat: f32, lon: f32, default: T) -> Self {
        let s = size as usize;
        OrthoProj{ _size: size,  _data: vec![default; s*s], _lat: lat.to_radians(), _lon: lon.to_radians() }
    }

    /// Create a new OrthoProj, `size` and `lon`/`lat`, but the background (non-sphere) is `bg`,
    /// and `surface` is used for values on the sphere.
    pub fn new_with_bg(size: u32, lat: f32, lon: f32, bg: T, surface: T) -> Self {
        let mut o = Self::new(size, lat, lon, bg);

        // draw a circle
        let r2 = (size/2)*(size/2);
        // presume centre is at (size/2, size/2)
        let cx = size/2;
        let cy = size/2;

        for x in 0..size {
            for y in 0..size {
                let dist2 = (x-cx)*(x-cx) + (y-cy)*(y-cy);
                if dist2 <= r2 {
                    o.set_pixel(x, y, surface.clone());
                }
            }
        }

        o
    }

    /// For this projection, what would be the pixel x/y values for this point. `None` if the
    /// lat/lon lies outside the visible area.
    pub fn xy_for_pos(&self, lat: f32, lon: f32) -> Option<(u32, u32)> {
        // lat = phi
        // lon = lambda
        //
        // x = r cos(lat)sin(lon - lon0)
        // y = r ( cos(lat0)sin(lat) - sin(lat0)cos(lat)cos(lon-lon0) )
        //
        // cos c = sin(lat0)sin(lat) + cos(lat0)cos(lat)cos(lon-lon0)
        // is it the far side of the globe
        let lat = lat.to_radians();
        let lon = lon.to_radians();
        let cos_c = self._lat.sin() * lat.sin() + self._lat.cos()*lat.cos()*(lon - self._lon).cos();
        if cos_c < 0. {
            return None;
        }

        let r = (self._size / 2) as f32;

        let x = r * lat.cos() * (lon - self._lon).sin();
        let y = r * ( self._lat.cos()*lat.sin() - self._lat.sin()*lat.cos()*(lon - self._lon).cos() );

        // FIXME Weird hack? Why is this required?
        let y = y * -1.;

        let x = x + r;
        let y = y + r;

        let x = x.trunc() as u32;
        let y = y.trunc() as u32;

        Some((x, y))
    }

    /// Set the value of `lat`, `lon` to `value`
    pub fn set(&mut self, lat: f32, lon: f32, value: T) {
        match self.xy_for_pos(lat, lon) {
            None => {},
            Some((x, y)) => {
                self.set_pixel(x, y, value);
            },
        };
    }

    /// For `lat`/`lon` what is the currently stored value?
    pub fn get(&self, lat: f32, lon: f32) -> &T {
        let r = (self._size / 2) as f32;
        let x = r * lat.to_radians().cos() * (lon - self._lon).to_radians().sin();
        let y = r * ( self._lon.to_radians().cos()*lat.to_radians().sin() - self._lon.to_radians().sin()*(lon - self._lat).to_radians().cos() );
        // FIXME clipping
        let i = x.trunc() as usize * self._size as usize + y.trunc() as usize;

        &self._data[i]
    }

    /// What is the current value of pixel `x`, `y`
    pub fn get_pixel(&self, x: u32, y: u32) -> &T {
        &self._data[(x*self._size+y) as usize]
    }

    /// Shortcut to set the value of pixel (`x`, `y`) to `value`.
    fn set_pixel(&mut self, x: u32, y: u32, value: T) {
        let i = x as usize * self._size as usize + y as usize;
        self._data[i] = value;
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_simple() {
        use super::OrthoProj;
        let o = OrthoProj::new(3, 0., 0., 0u8);
        assert_eq!(o.get_pixel(0, 0), &0u8);

    }
}
