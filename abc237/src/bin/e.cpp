#pragma GCC target("avx2")
#pragma GCC optimize("O3")
#pragma GCC optimize("unroll-loops")
#include<iostream>
#include<cstdint>
#include<cstddef>
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
constexpr i64 INF = 1e18;
class edge {
public:
	i64 from, to, cost;
};

template<typename T>
class Belman_Ford {
public:
	vec<edge> G;
	vec<i64> dist;
	bool flag;
	vec<bool> negative;

	Belman_Ford(const T &v, const T &e) {//vは頂点の数、eは辺の数
		G.resize(0);
		dist.resize(v, INF);
		negative.resize(v, false);
	}

	void init(vec<i64> &h,i64 loop) {//グラフの初期化(入力)
		rep(i, loop) {
			i64 a, b;
			cin >> a >> b;
			a--,b--;
			if (h[a] <= h[b]) {
			G.push_back({a,b,h[a] - h[b]});
			G.push_back({b,a,2 * (h[b] - h[a])});
			}else {
				swap(a,b);
				G.push_back({a,b,h[a] - h[b]});
			G.push_back({b,a,2 * (h[b] - h[a])});
			}
		}
	}

	void belman_ford(i64 v, i64 e, i64 s) {
		dist[s] = 0;
		rep(i, v - 1) {
			rep(j, e) {
				if (dist[G[j].from] == INF)continue;
				if (dist[G[j].to] > dist[G[j].from] + G[j].cost)
					dist[G[j].to] = dist[G[j].from] + G[j].cost;
			}
		}
		rep(i, v) {
			rep(j, e) {
				if (dist[G[j].to] == INF)continue;
				if (dist[G[j].to] > dist[G[j].from] + G[j].cost) {
					dist[G[j].to] = dist[G[j].from] + G[j].cost;
					negative[G[j].to] = 1;
				}
				if (negative[G[j].from] == true) {
					negative[G[j].to] = 1;
				}
			}
		}
	}
};

int main(){
	ios::sync_with_stdio(false);
	std::cin.tie(nullptr);
	i64 n,m;
	cin >> n >> m;
	vec<i64> h(n);
	rep(i,n)cin >> h[i],h[i] *= -1;
	Belman_Ford<i64> bel(n,m);
	bel.init(h,m);
	bel.belman_ford(n,2 * m,0);
	i64 ans = -INF;
	rep(i,n) {
		ans = max(-bel.dist[i],ans);
	}
	cout << ans << endl;
}
