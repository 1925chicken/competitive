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
void solve(){
}
int main(){
	ios::sync_with_stdio(false);
	std::cin.tie(nullptr);
	i64 n;
	cin >> n;
	vec<string> s(n),t(n);
	vec<P> s_pos,t_pos;
	rep(i,n){
		cin >> s[i];
		rep(j,n) {
			if(s[i][j] == '#') s_pos.push_back(P(j,i));
		}
	}
	rep(i,n) {
		cin >> t[i];
		rep(j,n) {
			if(t[i][j] == '#') t_pos.push_back(P(j,i));
		}
	}
	//cout << s_pos.size() << " " << t_pos.size() << endl;
	if(len(s_pos) != len(t_pos)) {
		cout << "No" << endl;
		return 0;
	}
	sort(all(s_pos));
	sort(all(t_pos));
	i64 a = s_pos[0].first;
	i64 b = s_pos[0].second;
	rep(i,len(s_pos)) {
		s_pos[i].first -= a;
		s_pos[i].second -= b;
	}
	a = t_pos[0].first;
	b = t_pos[0].second;
	rep(i,len(t_pos)) {
		t_pos[i].first -= a;
		t_pos[i].second -= b;
	}
	auto [tar_sx,tar_sy] = s_pos[0];
	rep(_,4) {
		sort(all(t_pos));
		auto [tar_tx,tar_ty] = t_pos[0];
		/*for(auto [x,y] : s_pos) {
			cout << "( " << x << ", " << y << " ), ";
		}
		for (auto [x,y] : t_pos) {
			cout << "( " << x << ", " << y << " ), ";
		}*/
		bool flag = true;
		rep(i,len(t_pos)) {
			if (tar_sx - s_pos[i].first != tar_tx - t_pos[i].first || tar_sy - s_pos[i].second != tar_ty - t_pos[i].second) flag = false;
		}
		if(flag) {
			cout << "Yes" << endl;
			return 0;
		}
		rep(i,len(t_pos)) {
			auto [x,y] = t_pos[i];
			i64 xx = -y;
			i64 yy = x;
			t_pos[i] = P(xx,yy);
		}
	}
	cout << "No" << endl;
}
