#pragma GCC target("avx2")
#pragma GCC optimize("O3")
#pragma GCC optimize("unroll-loops")
#include<iostream>
#include<cstdint>
#include<cstddef>
#include<set>
#include<algorithm>
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
	i64 n,m;
	cin >> n >> m;
	vec<P> ab = vec<P>(m);
	rep(i,m)cin >> ab[i].first >> ab[i].second;
	vec<i64> pos(n);
	rep(i,n)pos[i] = i;
	set<P> st;
	rep(i,m) {
		if(pos[ab[i].first - 1] > pos[ab[i].second - 1]) {
			if (st.count(ab[i])) {
				cout << "-1" << endl;
				return;
			}else {
				swap(pos[ab[i].first - 1], pos[ab[i].second - 1]);
			}
		}
		st.insert(P(ab[i].second, ab[i].first));
	}
	vec<i64> ans(n);
	rep(i,n) {
		ans[pos[i]] = i + 1;
	}
	rep(i,n) {
		cout << ans[i] << " \n"[i == n -1];
	}
}
int main(){
	ios::sync_with_stdio(false);
	std::cin.tie(nullptr);
	solve();
}
