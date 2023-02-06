/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: axlamber <axlamber@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/05 09:53:00 by rukkyaa           #+#    #+#             */
/*   Updated: 2023/02/06 10:13:38 by axlamber         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn deduplicate(list: &mut Vec<i32>) {
	let mut	i = 0;
	let mut j;

	while i < list.len() {
		j = i + 1;
		while j < list.len() {
			if list[j] == list[i] {list.remove(j);}
			else {j += 1}
        }
        i += 1
    }
}
