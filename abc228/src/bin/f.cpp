#pragma GCC target("avx2")
#pragma GCC optimize("O3")
#pragma GCC optimize("unroll-loops")
#include<iostream>
#include<cstdint>
#include<cstddef>
#include<vector>
#if __has_include(<atcoder/all>)
#include<atcoder/all>
using namespace atcoder;
#endif
//#include<boost/multiprecision/cpp_int.hpp>
//namespace mp = boost::multiprecision;
using namespace std;
using i32 = int_fast32_t;
using i64 = int_fast64_t;
using usize = size_t;
using u32 = uint_fast32_t;
using u64 = uint_fast64_t;
using i128 = __int128_t;
using u128 = __uint128_t;
using ld = long double;
template<typename T>
using vec = vector<T>;
#define rep(i, n) for (i64 i = 0; i < (i64)(n); ++i)
#define all(a) (a).begin(),(a).end()
#define rall(a) (a).rbegin(),(a).rend()
#define len(hoge) (i64)((hoge).size())
using P = pair<i64,i64>;
template<class T, class S> ostream &operator<<(ostream &os, const pair<T,S> &p){
	os << p.first << " " << p.second;
	return os;
}
void solve(){
}
//
// Created by chicken1925 on 2021/01/24.
//

class CumulativeSum_2D {
public:
	vec<vec<i64>> data;

	CumulativeSum_2D(i64 h, i64 w) : data(h + 1, vec<i64>(w + 1)) {}

	void add(i64 x, i64 y, i64 z) {
		x++, y++;
		if (y >= data.size() || x >= data[0].size())return;
		data[y][x] += z;
	}

	void build() {
		for (i32 i = 1; i < len(data); i++) {
			for (i32 j = 1; j < len(data[i]); j++) {
				data[i][j] += data[i -1][j] + data[i][j - 1] - data[i - 1][j - 1];
			}
		}
	}

	i64 query(i64 sy, i64 sx, i64 gy, i64 gx) {
		return data[gy][gx] - data[sy][gx] - data[gy][sx] + data[sy][sx];
	}
};

int main(){
	ios::sync_with_stdio(false);
	std::cin.tie(nullptr);
}
