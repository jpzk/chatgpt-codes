use chrono::NaiveDate;
fn main() {
}

// straight outta chatGPT
//
// Write me a rust function that takes a starting date and an end date and a list
// of date value tuples known as changes. All dates are of NaiveDate type. The
// values are f32 float values. The resulting timeline contains the changes but
// also all the missing values in between by assuming that the missing value is
// the last known value from the changes. The initial value is either the last
// change in terms of date that lies before the start date or 0.0. The resulting
// array should start at the start date argument and end at the end date
// argument. 

// And function that you should write should be called timeline. The first
// argument are the changes, second is start date, third is end date. Use
// references as input.
fn timeline(changes: &[(NaiveDate, f32)], start_date: &NaiveDate, end_date: &NaiveDate) -> Vec<(NaiveDate, f32)> {
    let mut filled_values = Vec::new();
    let mut current_value: f32 = 0.0;
    let mut current_date = start_date.clone();

    // Find the initial value
    for &(ref change_date, value) in changes.iter().rev() {
        if change_date < start_date {
            current_value = value;
            break;
        }
    }

    for &(ref change_date, value) in changes {
        if change_date >= start_date {
            while current_date < *change_date { // had to add * here
                filled_values.push((current_date.clone(), current_value));
                current_date = current_date.succ();
            }
            current_value = value;
            filled_values.push((change_date.clone(), current_value));
            current_date = change_date.succ();
        }
    }

    // Add the final value for the end date
    while current_date <= *end_date { // had to add * here
        filled_values.push((current_date.clone(), current_value));
        current_date = current_date.succ();
    }

    filled_values
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime};

    use crate::timeline;

    fn parse(time: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(time, "%Y-%m-%d %H:%M:%S").unwrap()
    }

    #[test]
    fn test_try_to_break() {
        let changes: Vec<(NaiveDate, f32)> = vec![
            (parse("2023-01-01 15:00:00").date(), 76.0),
        ];
        let start = parse("2022-11-17 15:29:28").date();
        let end = parse("2023-01-01 15:29:28").date();
        let result = timeline(changes.as_slice(),&start, &end);
        assert_eq!(result.len(), 46);

        let (last, before) = result.split_last().unwrap();
        let exp = (parse("2023-01-01 00:00:00").date(), 76.0);
        assert_eq!(last, &exp);
        before.iter().for_each(|b| { 
            assert_eq!(b.1, 0.0);
        })
       //assert_eq!(result, Vec::new())
    }


    #[test]
    fn test_from_start_to_including_end() {
        let changes: Vec<(NaiveDate, f32)> = Vec::new();
        let start = parse("2015-09-05 00:00:00").date();
        let end = parse("2015-09-07 00:00:00").date();
        let result = timeline(changes.as_slice(), &start, &end);
        assert_eq!(result.len(), 3);
        assert_eq!(result, vec![
            (parse("2015-09-05 00:00:00").date(), 0.0),
            (parse("2015-09-06 00:00:00").date(), 0.0),
            (parse("2015-09-07 00:00:00").date(), 0.0),
        ] )

    }

    #[test]
    fn test_intersected_changes() {
        let changes: Vec<(NaiveDate, f32)> = vec![
            (parse("2015-09-05 01:00:00").date(), 1.0),
            (parse("2015-09-06 05:00:00").date(), 2.0),
        ];
        let start = parse("2015-09-04 00:00:00").date();
        let end = parse("2015-09-07 00:00:00").date();

        let result = timeline(changes.as_slice(), &start, &end);
        assert_eq!(result.len(), 4);
        assert_eq!(
            result,
            vec![
                (parse("2015-09-04 00:00:00").date(), 0.0),
                (parse("2015-09-05 00:00:00").date(), 1.0),
                (parse("2015-09-06 00:00:00").date(), 2.0),
                (parse("2015-09-07 00:00:00").date(), 2.0),
            ]
        )
    }

    #[test]
    fn test_changes_before() {
        let changes: Vec<(NaiveDate, f32)> = vec![
            (parse("2015-09-03 00:30:00").date(), 1.0),
            (parse("2015-09-04 00:40:00").date(), 2.0),
        ];
        let start = parse("2015-09-05 01:00:00").date();
        let end = parse("2015-09-06 03:00:00").date();

        let result = timeline(changes.as_slice(), &start, &end);
        //assert_eq!(result.len(), 2);
        assert_eq!(
            result,
            vec![
                (parse("2015-09-05 00:00:00").date(), 2.0),
                (parse("2015-09-06 00:00:00").date(), 2.0),
            ]
        );
    }
}

