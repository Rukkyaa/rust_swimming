/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: axlamber <axlamber@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/03 11:31:58 by axlamber          #+#    #+#             */
/*   Updated: 2023/02/03 11:41:10 by axlamber         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::cmp::Ordering;

fn main()
{
    let price = ftkit::random_number(0..=10000);
    let mut guess = 10001;
    
    println!("Me and my infinite wisdom have found an appropriate secret you shall yearn for.");
    while guess != price
    {
        guess = ftkit::read_number();
        match guess.cmp(&price)
        {
            Ordering::Greater => println!("Sometimes I wonder whether I should retire. I would have guessed higher."),
            Ordering::Less => println!("This student might not be as smart as I was told. This answer is obviously too weak."),
            _ => println!("That is right! The secret was indeed the number {}, which you have brilliantly discovered!", price)
        }
    }
}
