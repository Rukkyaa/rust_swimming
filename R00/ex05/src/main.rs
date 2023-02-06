/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: axlamber <axlamber@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/02 14:33:42 by axlamber          #+#    #+#             */
/*   Updated: 2023/02/06 08:58:50 by axlamber         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn  is_leap_year(year: u32) -> bool
{
    if year == 0 {panic!("ERROR : 0 isnt a valid param");}
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn  num_days_in_month(year: u32, month: u32) -> u32
{
    if month == 2 && is_leap_year(year){return 29;}
    [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31][(month-1) as usize]
}

#[cfg(test)]
mod	test {
	use crate::num_days_in_month;
	use crate::is_leap_year;

	#[test]
	#[should_panic]
	fn	zero_month_test() {
		num_days_in_month(4242, 0);
	}

	#[test]
	#[should_panic]
	fn	too_much_month_test() {
		num_days_in_month(4242, 13);
	}

	#[test]
	#[should_panic]
	fn	year_zero_test() {
		is_leap_year(0);
	}

	#[test]
	fn	leap_year_test() {
		assert_eq!(is_leap_year(4), true);
		assert_eq!(is_leap_year(40), true);
		assert_eq!(is_leap_year(400), true);
		assert_eq!(is_leap_year(1600), true);
		assert_eq!(is_leap_year(2004), true);
		assert_eq!(is_leap_year(2000), true);
		assert_eq!(is_leap_year(1200), true);
	}

	#[test]
	fn	not_leap_year_test() {
		assert_eq!(is_leap_year(1), false);
		assert_eq!(is_leap_year(1500), false);
		assert_eq!(is_leap_year(2003), false);
		assert_eq!(is_leap_year(42), false);
		assert_eq!(is_leap_year(4100), false);
	}

	#[test]
	fn	num_day_test() {
		assert_eq!(num_days_in_month(1500, 1), 31);
		assert_eq!(num_days_in_month(1600, 1), 31);
		assert_eq!(num_days_in_month(1500, 2), 28);
		assert_eq!(num_days_in_month(1600, 2), 29);
		assert_eq!(num_days_in_month(1500, 3), 31);
		assert_eq!(num_days_in_month(1600, 4), 30);
		assert_eq!(num_days_in_month(1500, 5), 31);
		assert_eq!(num_days_in_month(1600, 6), 30);
		assert_eq!(num_days_in_month(1500, 7), 31);
		assert_eq!(num_days_in_month(1600, 8), 31);
		assert_eq!(num_days_in_month(1500, 9), 30);
		assert_eq!(num_days_in_month(1600, 10), 31);
		assert_eq!(num_days_in_month(1500, 11), 30);
		assert_eq!(num_days_in_month(1600, 12), 31);
	}
}

fn  main()
{
    let mut day_index = 1;
    let mut year = 0;
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    
    loop
    {
        year += 1;
        for month in 1..=12
        {
            for day in 1..=num_days_in_month(year, month)
            {
                if day_index == 8 {day_index = 1;}
                if day == 13 && day_index == 5
                {
                    println!("Friday, {} 13, {}", months[(month-1) as usize], year);
                }
                day_index += 1;
            }   
        }
    }

}

// fn main()
// {
//     println!("{}", is_leap_year(1600));
//     println!("{}", is_leap_year(2003));
//     println!("{}", is_leap_year(1500));
//     println!("{}", is_leap_year(2004));
//     println!("{}", num_days_in_month(2004, 12));
//     println!("{}", num_days_in_month(2003, 12));
// }
