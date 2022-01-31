#pragma GCC target("avx2")
#pragma GCC optimize("O3")
#pragma GCC optimize("unroll-loops")
#include<iostream>
#include<cstdint>
#include<cstddef>
#include<vector>
#include<algorithm>
#include<map>
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

map<P,i64> edge_cost;
map<i64,i64> mincost;
i64 max_cost;
template <typename T>
class UnionFind
{
public:
	vector<T> par;
	UnionFind(T n) : par(n, -1) {}
	void init(T n)
	{
		for (usize i = 0; i < n; i++)
		{
			par[i] = -1;
		}
	}

	i64 root(T x)
	{
		if (par[x] < 0)
			return x;
		else
			return par[x] = root(par[x]);
	}

	bool same(T x, T y)
	{
		return root(x) == root(y);
	}

	bool merge(T x, T y)
	{
		x = root(x);
		y = root(y);
		if (x == y)
			return false;
		if (par[x] > par[y])
			swap(x, y);
		par[x] += par[y];
		par[y] = x;
		return true;
	}

	i64 size(T x)
	{
		return -par[root(x)];
	}
};

template<typename T>
class edge{
public:
	T from,to,cost;
};

template<typename T>
T kruskal(vector<edge<T>> &edges,i64 V){
	sort(edges.begin(),edges.end(),[](const edge<T> &a,const edge<T> &b){
		return a.cost < b.cost;
	});
	UnionFind<i64> uft(V);
	T res = 0;
	for(edge<T> &e : edges){
		if(!(uft.same(e.from,e.to))){
			uft.merge(e.from,e.to);
			edge_cost[P(e.from,e.to)] = e.cost;
			res += e.cost;
			if (mincost.count(e.from)) {
				mincost[e.from] = min(mincost[e.from],e.cost);
			}else {
				mincost[e.from] = e.cost;
			}
			if (!mincost.count(e.to)) {
				mincost[e.to] = e.cost;
			}else {
				mincost[e.to] = min(mincost[e.to],e.cost);
			}
		}
	}
	return res;
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

	LCA(int_fast64_t _n) : n(_n),dig(32){
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
int main(){
	ios::sync_with_stdio(false);
	std::cin.tie(nullptr);
	i64 n,m,q;
	cin >> n >> m >> q;
	vec<edge<i64>> abc(m);
	rep(i,m){
		i64 a,b,c;
		cin >> a >> b >> c;
		a--,b--;
		abc[i] = {a,b,c};
	}
	kruskal(abc,n);
	LCA<i64> lca(n);
	for(auto [key,val]:edge_cost) {
		lca.addedge(key.first,key.second,val);
	}
	lca.construct();
	rep(i,q) {
		i64 a,b,c;
		cin >> a >> b >> c;
		a--,b--;
		if(a == b) {
			cout << "No" << endl;
		}else {
			cout << ((mincost[a] > c &&  mincost[b] > c && lca.dist(a,b) > c ) ? "Yes" :"No") << endl;
		}
	}
	
}
