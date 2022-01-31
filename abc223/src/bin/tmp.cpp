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
void solve(){
}
int main(){
	ios::sync_with_stdio(false);
	std::cin.tie(nullptr);
	i64 n;
	cin >> n;
	vec<P> ab(n);
	rep(i,n) cin >> ab[i].first >> ab[i].second;
	ld time = 0.0;
	rep(i,n){
		time += (ld(ab[i].first) / ld(ab[i].second));
	}
	//cout << fixed << setprecision(10) << time << endl;
	ld ans = 0.0;
	ld now_t = 0.0;
	rep(i,n) {
		if((now_t + ld(ab[i].first) / ld(ab[i].second)) * 2 >= time) {
			i64 cnt = 0;
				ld lb = 0.0,ub = ld(ab[i].first) + 1e-14;
				while (cnt < 100) {
					ld mid = (lb + ub) / 2;
					if (2 * (mid / ab[i].second + now_t) >= time) {
						lb = mid;
					}else {
						ub = mid;
					}
					cnt += 1;
				}
				cout << fixed << setprecision(10) << (ans + lb * ab[i].second) << endl;
				return 0;
		}else{
			ans += ab[i].first;
			now_t += ab[i].first / ab[i].second;
			//cout << ans << " " << now_t << endl;
		}
	}
}
