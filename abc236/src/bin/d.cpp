#pragma GCC target("avx2")
#pragma GCC optimize("O3")
#pragma GCC optimize("unroll-loops")
#include<iostream>
#include<cstdint>
#include<cstddef>
#include<vector>
#include<algorithm>
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
i64 ans;
i64 lim;
void solve(){
}
void dfs(i64 flag,vec<vec<i64>> &g,i64 now,i64 depth) {
	if (flag == lim) {
		ans = max(ans,now);
		return ;
	}
	if (flag & (1 << depth)) {
		dfs(flag,g,now,depth + 1);
	}else {
		flag |= ((i64)1 << depth);
		for(int i = depth + 1; i < len(g[depth]); i++) {
			if(!(flag & (1 << i))) {
				dfs(flag | ((i64)1 << i),g,now ^ g[depth][i],depth + 1);
			}
		}
	}
}
int main(){
	ios::sync_with_stdio(false);
	std::cin.tie(nullptr);
	i64 n;
	cin >> n;
	vec<vec<i64>> g(2 * n - 1);
	rep(i,2 * n - 1) {
		rep(j,2 * n) {
			if (i < j) {
				i64 b;
				cin >> b;
				g[i].emplace_back(b);
			}else {
				g[i].emplace_back(0);
			}
		}
	}
	rep(i,2 * n) {
		lim |= (1 << i);
	}
	dfs(0,g,0,0);
	cout << ans << endl;
}
