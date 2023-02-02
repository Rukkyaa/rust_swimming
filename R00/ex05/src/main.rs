/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: axlamber <axlamber@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/02 14:33:42 by axlamber          #+#    #+#             */
/*   Updated: 2023/02/02 18:03:06 by axlamber         ###   ########.fr       */
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
