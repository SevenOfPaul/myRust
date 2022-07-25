console.time("js");
let arr=[1,2,54,101,8,9,10];
for(let i=0;i<arr.length;i++){
    for(let j=0;j<arr.length;j++){
        if(arr[j]>arr[j+1]){
            [arr[j],arr[j+1]]=[arr[j+1],arr[j]];
        }
    }
}
console.log(arr)
console.timeEnd("js");