// Because Iam Happy!
// @author Jochen Saalfeld <jsaalfeld@uos.de>
// Codegolf Happy Numbers in JavaScript with 100 chars.

//v1 - function h(x){for(s=n=0;t=n<2e3;s=s>4?t-1:++n,s-1){if(++n==x){return true}}for each(c in''+s)t+=c*c; return false;}
function h(x){r=[];while(x!=1&&!r[x]){r[x]=1;t=0;while(x){n=x%10;t+=n*n;x=(x-n)/10}x=t}return x==1}

//output
i=0;while(i<100){console.log("Is "+i+" a happy number: "+h(i));i++}
