extern crate image;

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
        OrthoProj{ _size: size,  _data: vec![default; s*s], _lat: lat, _lon: lon }
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

    /// Set the value of `lat`, `lon` to `value`
    pub fn set(&mut self, lat: f32, lon: f32, value: T) {
        // lat = phi
        // lon = lambda
        //
        // x = r cos(lat)sin(lon - lon0)
        // y = r ( cos(lat0)sin(lat) - sin(lat0)cos(lat)cos(lon-lon0) )
        //
        // cos c = sin(lat0)sin(lat) + cos(lat0)cos(lat)cos(lon-lon0)
        // is it the far side of the globe
        let cos_c = self._lat.to_radians().sin() * lat.to_radians().sin() + self._lat.to_radians().cos()*lat.to_radians().cos()*(lon - self._lon).to_radians().cos();
        if cos_c < 0. {
            return;
        }

        let r = (self._size / 2) as f32;

        let x = r * lat.to_radians().cos() * (lon - self._lon).to_radians().sin();
        let y = r * ( self._lat.to_radians().cos()*lat.to_radians().sin() - self._lat.to_radians().sin()*lat.to_radians().cos()*(lon - self._lon).to_radians().cos() );

        // FIXME Weird hack? Why is this required?
        let y = y * -1.;

        let x = x + r;
        let y = y + r;


        self.set_pixel(x.trunc() as u32, y.trunc() as u32, value);
    }

    pub fn get(&self, lat: f32, lon: f32) -> &T {
        let r = (self._size / 2) as f32;
        let x = r * lat.to_radians().cos() * (lon - self._lon).to_radians().sin();
        let y = r * ( self._lon.to_radians().cos()*lat.to_radians().sin() - self._lon.to_radians().sin()*(lon - self._lat).to_radians().cos() );
        // FIXME clipping
        let i = x.trunc() as usize * self._size as usize + y.trunc() as usize;

        &self._data[i]
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> &T {
        &self._data[(x*self._size+y) as usize]
    }

    fn set_pixel(&mut self, x: u32, y: u32, value: T) {
        let i = x as usize * self._size as usize + y as usize;
        self._data[i] = value;
    }


    pub fn image<P, Container> (&self, f: &Fn(&T) -> image::Pixel) -> image::ImageBuffer<P, Container> where P: image::Pixel {
        let size = self._size as usize;
        image::ImageBuffer::from_fn(size, size, |x, y| f(self.get_pixel(x, y)) );

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
