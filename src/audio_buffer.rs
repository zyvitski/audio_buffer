#![allow(dead_code)]

#[derive(Debug)]
pub struct AudioBuffer<'a, T>
where
    [T]: 'a,
    T: 'a,
{
    data: &'a [T],
    chans: usize,
}

impl<'a, T> AudioBuffer<'a, T>
where
    [T]: 'a,
    T: 'a,
{
    pub fn new(data: &'a [T], chans: usize) -> Self {
        AudioBuffer {
            data: data,
            chans: chans,
        }
    }

    pub fn get(&self, index: usize) -> Option<&'a [T]> {
        let idx = index * self.chans;
        if idx < self.data.len() {
            unsafe { Some(::std::slice::from_raw_parts(&self.data[idx], self.chans)) }
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.data.len() / self.chans
    }

    pub fn width(&self) -> usize {
        self.chans
    }

    pub fn iter(&'a self) -> FrameIterator<'a, T> {
        FrameIterator { data: self, idx: 0 }
    }
}


pub struct FrameIterator<'a, T>
where
    [T]: 'a,
    T: 'a,
{
    data: &'a AudioBuffer<'a, T>,
    idx: usize,
}

impl<'a, T> Iterator for FrameIterator<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.idx;
        self.idx += 1;
        self.data.get(i)
    }
}

#[derive(Debug)]
pub struct AudioBufferMut<'a, T>
where
    [T]: 'a,
    T: 'a,
{
    data: &'a mut [T],
    chans: usize,
}

impl<'a, T> AudioBufferMut<'a, T>
where
    [T]: 'a,
    T: 'a,
{
    pub fn new(data: &'a mut [T], chans: usize) -> Self {
        AudioBufferMut {
            data: data,
            chans: chans,
        }
    }

    pub fn get(&self, index: usize) -> Option<&'a [T]> {
        let idx = index * self.chans;
        if idx < self.data.len() {
            unsafe { Some(::std::slice::from_raw_parts(&self.data[idx], self.chans)) }
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&'a mut [T]> {
        let idx = index * self.chans;
        if idx < self.data.len() {
            unsafe {
                Some(::std::slice::from_raw_parts_mut(
                    &self.data[idx] as *const T as *mut T,
                    self.chans,
                ))
            }
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.data.len() / self.chans
    }

    pub fn width(&self) -> usize {
        self.chans
    }

    pub fn iter_mut(&'a mut self) -> FrameMutIterator<'a, T> {
        FrameMutIterator { data: self, idx: 0 }
    }
}


pub struct FrameMutIterator<'a, T>
where
    [T]: 'a,
    T: 'a,
{
    data: &'a mut AudioBufferMut<'a, T>,
    idx: usize,
}

impl<'a, T> Iterator for FrameMutIterator<'a, T> {
    type Item = &'a mut [T];
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.idx;
        self.idx += 1;
        self.data.get_mut(i)
    }
}
