#pragma GCC target("avx2")
#pragma GCC optimize("O3")
#pragma GCC optimize("unroll-loops")
#include<iostream>
#include<cstdint>
#include<map>
#include<algorithm>
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
//
// Created by chicken1925 on 2021/01/22.
//

template<typename T>
class LCA {
public:
	vector<int_fast32_t> depth;
	const int_fast32_t dig;
	vector<vector<int_fast32_t>> g;
	vector<vector<int_fast32_t>> cost;
	vector<int_fast32_t> costs;
	int_fast32_t n;
	vector<vector<int_fast32_t>> doubling;

	LCA(int_fast64_t _n) : n(_n),dig(64){
		doubling.assign(dig, vector<int_fast32_t>(n, -1));
		costs.resize(n);
		depth.resize(n);
		g.resize(n);
		cost.resize(n);
	}

	void addedge(int_fast32_t u, int_fast32_t v, int_fast32_t c = 0) {
		g[u].emplace_back(v);
		cost[u].emplace_back(c);
		g[v].emplace_back(u);
		cost[v].emplace_back(c);

	}

	void dfs(int_fast32_t now, int_fast32_t par = -1, int_fast32_t d = 0, int_fast32_t c = 0) {
		doubling[0][now] = par;
		depth[now] = d;
		costs[now] = c;
		rep(i, len(g[now])) {
			if (g[now][i] != par)dfs(g[now][i], now, d + 1, c + cost[now][i]);
		}
	}

	void construct() {
		dfs(0);
		for (int_fast32_t i = 0; i < dig - 1; i++) {
			for (int_fast32_t j = 0; j < doubling[i].size(); j++) {
				if (doubling[i][j] == -1)doubling[i + 1][j] = -1;
				else doubling[i + 1][j] = doubling[i][doubling[i][j]];
			}
		}
	}

	int_fast32_t query_answer(int_fast32_t u, int_fast32_t v) {
		if (depth[u] > depth[v])swap(u, v);
		for (int_fast32_t i = dig - 1; 0 <= i; i--) {
			if (((depth[v] - depth[u]) >> i) & 1) v = doubling[i][v];
		}
		if (u == v) return u;
		for (int_fast32_t i = dig - 1; 0 <= i; --i) {
			if (doubling[i][u] != doubling[i][v]) {
				u = doubling[i][u];
				v = doubling[i][v];
			}
		}
		return doubling[0][u];
	}

	int_fast32_t length(int_fast32_t u, int_fast32_t v) {
		return depth[u] + depth[v] - 2 * depth[query_answer(u, v)];
	}

	int_fast32_t dist(int_fast32_t u, int_fast32_t v) {
		return costs[u] + costs[v] - 2 * costs[query_answer(u, v)];
	}

	bool isOnPath(int_fast32_t u, int_fast32_t v, int_fast32_t x) {
		return length(u, x) + length(v, x) == length(u, v);
	}
};
//based on https://satanic0258.github.io/snippets/tree/EulerTour.html
//verified on https://atcoder.jp/contests/abc138/submissions/19415353


class Euler_Tour {
public:
	std::vector<uint_fast64_t> euler_tour;
	std::vector<uint_fast64_t> begin, end;//[begin, end)の半開区間
	int_fast64_t n, k;

	Euler_Tour(const std::vector<std::vector<uint_fast64_t>> &g, uint_fast64_t v) : n(g.size()), k(0) {
		begin.resize(n, 0);
		end.resize(n , 0);
		//depth.resize(n, 0);
		build(g, v, -1, 0);
	}

	void build(const std::vector<std::vector<uint_fast64_t>> &g, uint_fast64_t v, int_fast64_t par, uint_fast64_t d) {
		begin[v] = k++;
		euler_tour.emplace_back(v);
		for (auto to :g[v]) {
			if (to == par)continue;
			build(g, to, v, d + 1);
			//depth[k] = d;
			euler_tour.emplace_back(v);
			k++;
		}
		end[v] = k;
	}
};


void solve(){
}
int main(){
	ios::sync_with_stdio(false);
	std::cin.tie(nullptr);
	u64 n;
	cin >> n;
	map<P,i64> mp;
	LCA<i64> lca(n);
	vec<vec<u64>> g(n);
	rep(i,n - 1) {
		u64 a,b;
		i64 c;
		cin >> a >> b >> c;
		lca.addedge(a - 1,b - 1,c);
		a -= 1,b -= 1;
		g[a].emplace_back(b);
		g[b].emplace_back(a);
		mp[P(min(a,b),max(a,b))] = c;
	}
	vec<i64> d(n);
	rep(i,n)cin >> d[i];
	Euler_Tour et(g,0);
	lca.construct();
	/*for(auto v:et.euler_tour) {
		cout << v << " ";
	}
	cout << endl;*/
	vec<i64> parity(n);
	map<i64,i64> m2;
	vec<i64> sum(2 * n);
	rep(i,n) {
		sum[et.begin[i]] += d[i];
		sum[et.end[i]] -= d[i];
		m2[et.begin[i]] = i; 
	}
	rep(i,2 * n - 1) {
		sum[i + 1] += sum[i];
	}
	/*for (auto v:sum) {
		cout << v << " ";
	}
	cout << endl;*/
	vec<P> rec(n);
	map<i64,i64> sum_to_key;
	rep(i,n) {
		rec[i] = P(sum[et.begin[i]],m2[et.begin[i]]);
		sum_to_key[sum[et.begin[i]]] = i;
	}
	/*for(auto [key,val]:rec) {
		cout << key << " ";
	}
	cout << endl;*/
	vec<i64> max_key_back(n);
	vec<i64> max_val_fro(n);
	vec<i64> max_val_back(n);
	max_val_back[n - 1] = rec[n - 1].first;
	max_key_back[n - 1] = rec[n - 1].second;
	for (i64 i = n - 2; 0 < i; i--) {
		if (rec[i].first > max_val_back[i + 1]) {
			max_key_back[i] = sum_to_key[rec[i].first];
			max_val_back[i] = rec[i].first;
		}
	}
	vec<i64> max_key_fro(n);
	max_val_fro[0] = rec[0].first;
	rep(i,n - 1) {
		if (rec[i + 1].first > max_val_fro[i]) {
			
		}
	}
}
