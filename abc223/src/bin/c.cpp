#pragma GCC target("avx2")
#pragma GCC optimize("O3")
#pragma GCC optimize("unroll-loops")
#include<iostream>
#include<cstdint>
#include<cstddef>
#include<iomanip>
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
bool check(vec<P> ab,ld time) {
	ld now_l = 0.0;
	ld now_r = 0.0;
	
}

ld bs(vec<P> ab) {
	ld lb = 0.0,ub = 1e18;//seconds needed
	i64 cnt = 0;
	while (cnt < 100) {
		ld mid = (lb + ub) / 2;
		if(check(ab,mid)) {
			ub = mid;
		}else {
			lb = mid;
		}
		cnt += 1;
	}
	return ub;
}

void solve(){
	i64 n;
	cin >> n;
	vec<P> ab = vec<P>(n);
	rep(i,n)cin >> ab[i].first >> ab[i].second;
	cout << fixed << setprecision(10) << bs(ab) << endl;
}
int main(){
	ios::sync_with_stdio(false);
	std::cin.tie(nullptr);
	solve();
}
