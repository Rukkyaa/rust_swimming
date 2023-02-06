/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: axlamber <axlamber@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/03 16:18:10 by axlamber          #+#    #+#             */
/*   Updated: 2023/02/06 11:13:43 by axlamber         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn verif_next_pattern(query: &[u8], pattern: &[u8]) -> bool {
	let mut i = 0;
	let p_len = pattern.len();

	while i < p_len && pattern[i] != b'*'
	{
		if pattern[i] != query[i] {return false;}
		i += 1;
	}
	return true;
}

pub fn strpcmp(query: &[u8], pattern: &[u8]) -> bool {
	let mut p = 0;
	let mut q = 0;
	let p_len = pattern.len();
	let q_len = query.len();

	while p < p_len && q < q_len
	{
		if pattern[p] == b'*'
		{
			if p == p_len - 1 {return true;} // * at the end
			while q < q_len
			{
				if verif_next_pattern(&query[q..], &pattern[p + 1..])
				{
					if strpcmp(&query[q..], &pattern[p + 1..])
					{
						return true;
					}
				}
				q += 1
			}
			return false;
		}
		else
		{
			if pattern[p] != query[q] {return false;}
		}
		p += 1;
		q += 1;
	}
	while p != p_len && pattern[p] == b'*' { p += 1; }
	p == p_len && q == q_len
}

#[test]
fn ultimate() {
    assert!(strpcmp(b"abbcdefcdeeeffef", b"ab*cd*ef"));
}
