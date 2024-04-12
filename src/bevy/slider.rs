use bevy::a11y::accesskit::NodeBuilder;
use bevy::prelude::*;
use std::ops::Range;

#[derive(Bundle)]
struct SliderBundel<T>
where
    T: PartialOrd + Sync + Send + 'static,
{
    pub transform: Transform,
    pub track: SliderTrack<T>,
    pub thumb: SliderThumb<T>,
}
impl<T: Default + PartialOrd + Send + Sync + 'static> Default for SliderBundel<T> {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            track: Default::default(),
            thumb: Default::default(),
        }
    }
}

#[derive(Component)]
pub struct SliderTrack<T> {
    pub range: Range<T>,
}
impl<T: Default> Default for SliderTrack<T> {
    fn default() -> Self {
        Self {
            range: Default::default(),
        }
    }
}

#[derive(Component)]
pub struct SliderThumb<T> {
    pub value: T,
    pub transform: Transform,
}
impl<T: Default> Default for SliderThumb<T> {
    fn default() -> Self {
        Self {
            value: Default::default(),
            transform: Default::default(),
        }
    }
}
impl<T> SliderThumb<T> {
    fn get_value_ref<'a>(&'a self) -> &'a T {
        &self.value
    }
}
impl<T: Clone> SliderThumb<T> {
    fn get_value_cloned(&self) -> T {
        self.value.clone()
    }
}
