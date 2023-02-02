/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   other.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: axlamber <axlamber@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/01 16:58:00 by axlamber          #+#    #+#             */
/*   Updated: 2023/02/02 13:58:43 by axlamber         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn	main()
{
	println!("Hey! I'm the other bin target!");
    #[cfg(not(debug_assertions))]
    println!("I'm in release mode!");
}