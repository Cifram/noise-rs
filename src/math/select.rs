/// This macro blends between any number of values based on a single control
/// value. The syntax for calling it looks like:
///
/// select!(control,
///     {
///         value1
///     } => blend_min1, blend_max1 => {
///         value2
///     } => blend_min2, blend_max2 => {
///         value3
///     }
/// )
///
/// if the control is between the two values in a blend_min and blend_max pair,
/// it will blend between the previous and next values. Otherwise, it will just
/// use the appropriate value. Note that it will only execute the blocks it
/// needs, which will always be 1 or 2 blocks.
#[macro_export]
macro_rules! select(
    (
        $control:expr, $value0:block =>
        $range_start1:expr, $range_end1:expr => $value1:block
    ) => {
        if $control < $range_start1 {
            $value0
        } else if $control < $range_end1 {
            let alpha = (($control - $range_start1) / ($range_end1 - $range_start1));
            let alpha = alpha * alpha * (3.0 - (alpha * 2.0));
            let a = $value0;
            let b = $value1;
            b * alpha + a * (1.0 - alpha)
        } else {
            $value1
        }
    };
    (
        $control:expr, $value0:block =>
        $range_start1:expr, $range_end1:expr => $value1:block =>
        $($range_start:expr, $range_end:expr => $value:block) =>+
    ) => {
        if $control < $range_start1 {
            $value0
        } else if $control < $range_end1 {
            let alpha = (($control - $range_start1) / ($range_end1 - $range_start1));
            let alpha = alpha * alpha * (3.0 - (alpha * 2.0));
            let a = $value0;
            let b = $value1;
            b * alpha + a * (1.0 - alpha)
        } else {
            select!(
                $control, $value1 =>
                $($range_start, $range_end => $value) =>+
            )
        }
    };
);
