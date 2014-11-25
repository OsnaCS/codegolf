// Because Iam Happy!
// @author Jochen Saalfeld <jsaalfeld@uos.de>
// Codegolf Happy Numbers in JavaScript with 96 chars.

//v1 - function h(x){for(s=n=0;t=n<2e3;s=s>4?t-1:++n,s-1){if(++n==x){return true}}for each(c in''+s)t+=c*c; return false;}
//v2 - function h(x){r=[];while(x!=1&&!r[x]){r[x]=1;t=0;while(x){n=x%10;t+=n*n;x=(x-n)/10}x=t}return x==1}
function h(f){for(r=[];1!=f&&!r[f];){for(r[f]=1,t=0;f;)n=f%10,t+=n*n,f=(f-n)/10;f=t}return 1==f}

//output
i=0;while(i<100){console.log("Is "+i+" a happy number: "+h(i));i++}
