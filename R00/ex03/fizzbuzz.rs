/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   fizzbuzz.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: axlamber <axlamber@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/01 16:39:38 by axlamber          #+#    #+#             */
/*   Updated: 2023/02/02 10:34:42 by axlamber         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn	main()
{
	for i in 1..=100
	{
		match (i % 3, i % 5, i % 11)
		{
			(0, 0, _) => println!("fizzbuzz"),
			(0, _, _) => println!("fizz"),
			(_, 0, _) => println!("buzz"),
			(_, _, 3) => println!("FIZZ"),
			(_, _, 5) => println!("BUZZ"),
			_ => println!("{}", i)
		}
	}
}