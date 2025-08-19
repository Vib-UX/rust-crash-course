pub fn first<T,M>(t: (T, M)) -> T {
    let (first, _) =t;
    return first;
}

pub fn last<T,M>(t: (T, M)) -> M {
    let (_, last) = t;
    return last;
}

#[derive(Debug)]
pub struct Rectangle {
    pub top: u32,
    pub left: u32,
    pub width: u32,
    pub height: u32,
}
