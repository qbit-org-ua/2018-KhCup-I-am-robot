#include <bits/stdc++.h>
#define int long long
using namespace std;
char mas[9][9];
int curPosI, curPosJ;
int score;
vector<pair<int, int> > balls, spaces;
bool used[10][10];
int bfs[10][10];
int di[8]={0, 0, 1, -1, 1, 1, -1, -1};
int dj[8]={-1, 1, 0, 0, -1, 1, -1, 1};
int f[6]={0, 10, 1000, 100000, 10000000, 1000000000};
bool check(int i, int j)
{
    return (i>=0 && i<9 && j>=0 && j<9 && !used[i][j] && mas[i][j]=='_');
}
bool check2(int i, int j)
{
    return (i>=0 && i<9 && j>=0 && j<9);
}
int priority(int ci, int cj)
{
        int Max=0;
        for (int si=0; si<9; si++)
        {
            for (int sj=0; sj<9; sj++)
            {
                for (int k=0; k<8; k++)
                {
                    int ii = si;
                    int jj = sj;
                    bool boo = 0;
                    bool boo2 = 1;
                    int col=0;
                    string s="";
                    while (ii>=0 && ii<9 && jj>=0 && jj<9 && mas[ii][jj]==mas[si][sj])
                    {
                        if (ii==ci && jj==cj)
                            boo = 1;
                        boo2 = 0;
                        col++;
                        ii+=di[k];
                        jj+=dj[k];
                    }
                    int fff = f[col];
                    if (!boo2)
                    {
                        fff/=3;
                    }
                    if (boo){
                        Max = max(Max, fff);
                    }

                }
            }
        }
    return Max;
}
void func_bfs(int startI, int startJ)
{
    queue<pair<int, int> > q;
    q.push({startI, startJ});
    while (!q.empty())
    {
        int i = q.front().first;
        int j = q.front().second;
        for (int k=0; k<4; k++)
        {
            int i1 = i+di[k];
            int j1 = j+dj[k];
            if (check(i1, j1))
            {
                used[i1][j1] = 1;
                q.push({i1, j1});
            }
        }
        q.pop();
    }
}
signed main()
{
    for (int i=0; i<9; i++)
    {
        for (int j=0; j<9; j++)
        {
            cin >> mas[i][j];
        }
    }
    cin >> score;
    for (int i=0; i<9; i++)
    {
        for (int j=0; j<9; j++)
        {
            if (mas[i][j]!='_')
                balls.push_back({i,j});
        }
    }
    int Max=0,ir1=0,jr1=0, ir2=0,jr2=0;
    for (int i=0; i<balls.size(); i++)
    {

        for (int j=0; j<9; j++)
            for (int k=0; k<9; k++)
                used[j][k] = 0, bfs[j][k] = 0;
        int is = balls[i].first, js = balls[i].second;

        func_bfs(is, js);
        char tch = mas[is][js];
        for (int j=0; j<9; ++j)
        {
            for (int k=0; k<9;k++)
            {
                if (used[j][k] && mas[j][k]=='_')
                {
                    mas[is][js] = '_';
                    mas[j][k] = tch;
                    curPosI = is;
                    curPosJ = js;
                    if (priority(j, k)>Max){
                        Max = priority(j, k);
                        ir1 = is;
                        jr1 = js;
                        ir2 = j;
                        jr2 = k;
                    }
                    mas[j][k] = '_';
                    mas[is][js] = tch;
                }
            }
        }


        }
    cout << jr1+1<<' '<<ir1+1<<' '<<jr2+1<<' '<<ir2+1<<endl;
    return 0;
}
