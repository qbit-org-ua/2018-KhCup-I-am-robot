#include <iostream>
#include <string>
#include <algorithm>
#include <fstream>
#include <vector>
using namespace std;

long long horosh(string **mas)
{
    long long res=0;
    for(int x=0;x<9;x++)
    {
        for(int y=0;y<5;y++)
        {
            int r,g,b,yc,p,m,c,_p;r=0;g=0;b=0;yc=0;p=0;m=0;c=0;_p=0;
            for(int i=0;i<5;i++)
            {

                    if(mas[x][y+i]== "R")r++;
                    if(mas[x][y+i]== "G")g++;
                    if(mas[x][y+i]== "C")c++;
                    if(mas[x][y+i]== "P")p++;
                    if(mas[x][y+i]== "B")b++;
                    if(mas[x][y+i]== "M")m++;
                    if(mas[x][y+i]== "Y")yc++;
                    if(mas[x][y+i]== "_")_p++;


            }
             if(r+g+b+yc+p+m+c==max(max(max(max(max(max(r,g),b),yc),p),m),c))
                {
                    int sum=r+g+b+yc+p+m+c;
                    //if(sum==1) cout << x << ' ' << y << " 1" <<endl;
                    if(sum==0) res++;
                    if(sum==1) res+=100;
                    if(sum==2) res+=10000;
                    if(sum==3) res+=1000000;
                    if(sum==4) res+=100000000;
                    if(sum==5) res+=100000000000;
                }
        }
    }
    for(int y=0;y<9;y++)
    {
        for(int x=0;x<5;x++)
        {
            int r,g,b,yc,p,m,c,_p;r=0;g=0;b=0;yc=0;p=0;m=0;c=0;_p=0;
            for(int i=0;i<5;i++)
            {

                    if(mas[x+i][y]== "R")r++;
                    if(mas[x+i][y]== "G")g++;
                    if(mas[x+i][y]== "C")c++;
                    if(mas[x+i][y]== "P")p++;
                    if(mas[x+i][y]== "B")b++;
                    if(mas[x+i][y]== "M")m++;
                    if(mas[x+i][y]== "Y")yc++;
                    if(mas[x+i][y]== "_")_p++;


            }
                            if(r+g+b+yc+p+m+c==max(max(max(max(max(max(r,g),b),yc),p),m),c))
                {
                    int sum=r+g+b+yc+p+m+c;
                    //if(sum==1) cout << x << ' ' << y << " 2" <<endl;
                    //if(x==4 && y==0) cout << "#"<<sum;
                    if(sum==0) res++;
                    if(sum==1) res+=100;
                    if(sum==2) res+=10000;
                    if(sum==3) res+=1000000;
                    if(sum==4) res+=100000000;
                    if(sum==5) res+=100000000000;
                }
        }
    }
      for(int y=0;y<5;y++)
    {
        for(int x=0;x<5;x++)
        {
            int r,g,b,yc,p,m,c,_p;r=0;g=0;b=0;yc=0;p=0;m=0;c=0;_p=0;
            for(int i=0;i<5;i++)
            {

                    if(mas[x+i][y+i]== "R")r++;
                    if(mas[x+i][y+i]== "G")g++;
                    if(mas[x+i][y+i]== "C")c++;
                    if(mas[x+i][y+i]== "P")p++;
                    if(mas[x+i][y+i]== "B")b++;
                    if(mas[x+i][y+i]== "M")m++;
                    if(mas[x+i][y+i]== "Y")yc++;
                    if(mas[x+i][y+i]== "_")_p++;


            }
                            if(r+g+b+yc+p+m+c==max(max(max(max(max(max(r,g),b),yc),p),m),c))
                {
                    int sum=r+g+b+yc+p+m+c;
                    //if(sum==1) cout << x << ' ' << y << " 3" <<endl;
                    if(sum==0) res++;
                    if(sum==1) res+=100;
                    if(sum==2) res+=10000;
                    if(sum==3) res+=1000000;
                    if(sum==4) res+=100000000;
                    if(sum==5) res+=100000000000;
                }
        }

    }
          for(int y=4;y<9;y++)
    {
        for(int x=0;x<5;x++)
        {
            int r,g,b,yc,p,m,c,_p;r=0;g=0;b=0;yc=0;p=0;m=0;c=0;_p=0;
            for(int i=0;i<5;i++)
            {

                    if(mas[x+i][y-i]== "R")r++;
                    if(mas[x+i][y-i]== "G")g++;
                    if(mas[x+i][y-i]== "C")c++;
                    if(mas[x+i][y-i]== "P")p++;
                    if(mas[x+i][y-i]== "B")b++;
                    if(mas[x+i][y-i]== "M")m++;
                    if(mas[x+i][y-i]== "Y")yc++;
                    if(mas[x+i][y-i]== "_")_p++;


            }
                if(r+g+b+yc+p+m+c==max(max(max(max(max(max(r,g),b),yc),p),m),c))
                {
                    int sum=r+g+b+yc+p+m+c;
                    if(sum==0) res++;
                    //if(sum==1) cout << x << ' ' << y << " 4" <<endl;
                    if(sum==1) res+=100;
                    if(sum==2) res+=10000;
                    if(sum==3) res+=1000000;
                    if(sum==4) res+=100000000;
                    if(sum==5) res+=100000000000;
                }
        }
    }
    return res;
}
int tablsv[9][9];
int parent(int x,int y)
{
    if(tablsv[x][y]==9*x+y) return (9*x+y);
    int k=parent(tablsv[x][y]/9,tablsv[x][y]%9);
    tablsv[x][y]=k;
    return k;
}
void unite(int x1,int y1,int x2,int y2)
{
    int c=parent(x1,y1);
    int d=parent(x2,y2);
    if(c!=d) tablsv[c/9][c%9]=tablsv[d/9][d%9];
}
int main()
{
    string **mas=new string *[9];
    for(int i=0;i<9;i++)mas[i]=new string[9];
    for(int x=0;x<9;x++) for(int y=0;y<9;y++) cin >> mas[x][y];

    for(int x=0;x<9;x++) for(int y=0;y<9;y++) tablsv[x][y]=x*9+y;
    for(int x1=0;x1<9;x1++) for(int y1=0;y1<9;y1++) for(int x2=0;x2<9;x2++) for(int y2=0;y2<9;y2++) if(mas[x1][y1]==mas[x2][y2] && mas[x2][y2]=="_" && (abs(x1-x2)+abs(y1-y2)==1)) unite(x1,y1,x2,y2);
    vector <pair <int,int> > shary,pust;
     for(int x=0;x<9;x++)
     {
        for(int y=0;y<9;y++)
            {
                if(mas[x][y]=="_") pust.push_back(make_pair(x,y));
        else shary.push_back(make_pair(x,y));
            }
     }
     long long Max=0; pair <int,int> fr,to;
     for(int i=0;i<shary.size();i++)
     {
         int x=shary[i].first;int y=shary[i].second;
         for(int z=0;z<pust.size();z++)
        {
            int xt=pust[z].first;int yt=pust[z].second;
         bool was=false;
         for(int xd=-1;xd<2;xd++)
         {
             for(int yd=-1;yd<2;yd++)
             {
                 if(abs(xd)+abs(yd)==1)
                 {
                     int xs=x+xd;
                     int ys=y+yd;
                     if(xs>=0 && ys>=0 && xs<9 && ys<9 && mas[xs][ys]=="_" && parent(xs,ys)==parent(xt,yt)) was=true;
                 }
             }
         }
         swap(mas[x][y],mas[xt][yt]);
         if(was)
         {
             long long zn=horosh(mas);
             if(zn>=Max)
             {
                 Max=zn;
                 fr.first=x;fr.second=y;to.first=xt;to.second=yt;
             }
         }
         swap(mas[x][y],mas[xt][yt]);
     }
     }
     //cin >> Max;
      cout << fr.second+1<<' '<<fr.first+1<<' '<<to.second+1<< ' '<<to.first+1;
    return 0;
}
