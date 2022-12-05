/**
use std::ptr::{Unique,self};
untie
#[derive(Debug)]
struct Unique<T> {
    ptr: *const T,
    // _marker:PhantomData<T>
}
unsafe impl <T:Send> Send for Unique<T>{}
unsafe impl <T:Sync> Sync for Unique<T>{}
impl<T> Unique<T>{
    const unsafe fn new_checked(ptr:*mut T)->Self{
        unsafe {
            Unique{ ptr}
        }
    }
    fn new(ptr:*mut T)->Option<Self>{
        if !ptr.is_null(){
          Some(Unique{ ptr})
        }else{
            None
        }
    }
    fn as_ptr(&self)-> *mut T{
        self.ptr as *mut T
    }
}
**/
use std::{mem, ptr, slice};
use std::alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout};
use std::ops::{Deref, DerefMut};
use std::path::Iter;
use std::ptr::NonNull;
#[derive(Debug)]
struct MVec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}
impl<T> MVec<T> {
    fn new() -> Self {
        assert_ne!(mem::size_of::<T>(), 0, "零尺寸类型无法处理");
        MVec {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }
    //内存分配
    fn grow(&mut self) {
        unsafe {
            //获取到类型对齐方式
            let align = mem::align_of::<T>();
            let elem_size = mem::size_of::<T>();
            let layout: Layout;
            let (new_cap, ptr) = if self.cap == 0 {
                layout = Layout::from_size_align_unchecked(elem_size, align);
                let ptr = alloc(layout);
                (1, ptr)
            } else {
                let new_cap = self.cap * 2;
                let old_num_bytes = self.cap * elem_size;
                assert!(old_num_bytes <= (isize::MAX as usize) / 2, "容量溢出");
                let new_num_bytes = old_num_bytes * 2;
                layout = Layout::from_size_align_unchecked(new_num_bytes, align);
                let ptr = realloc(self.ptr.as_ptr() as *mut _, layout, new_num_bytes);
                (new_cap, ptr)
            };
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            if let Some(ptr) = NonNull::new(ptr as *mut _) {
                self.ptr = ptr;
            } else {
                panic!("error");
            }
        }
    }
    unsafe fn push(&mut self, val: T) {
        if self.len == self.cap {
            self.grow();
        }
        unsafe {
            ptr::write(self.ptr.as_ptr().offset(self.len as isize), val);
        }

        self.len += 1;
    }
    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr.as_ptr().offset(self.len as isize))) }
        }
    }
    fn get(&self, index: usize) -> Option<T> {
        if index < self.len {
            unsafe {
                //这里count会直接获取到size
                Some(ptr::read(self.ptr.as_ptr().offset(index as isize)))
            }
        } else {
            None
        }
    }
    //----------------
    fn insert(&mut self, pos: usize, val: T) {
        assert!(pos < self.len, "下标越界");
        if self.cap == self.len {
            self.grow();
        }
        unsafe {
            ptr::copy(
                //移动哪里的内存
                self.ptr.as_ptr().offset(pos as isize),
                //移动到内存
                self.ptr.as_ptr().offset((pos + 1) as isize),
                self.len - pos as usize,
            );
            ptr::write(self.ptr.as_ptr().offset(pos as isize), val);
        }
        self.len += 1;
    }
    fn remove(&mut self, pos: usize) {
        assert!(pos >= 0 || pos < self.len, "下标越界");
        unsafe {
            ptr::copy(
                //移动哪里的内存
                self.ptr.as_ptr().offset((pos + 1) as isize),
                //移动到内存
                self.ptr.as_ptr().offset(pos as isize),
                self.len - pos as usize,
            );
            self.len -= 1;
        }
    }
    fn into_iter(self)->IntoIter<T>{
        let ptr=self.ptr;
        let cap=self.cap;
        let len=self.len;
        mem::forget(self);
        unsafe {
            IntoIter{
                buf:ptr,
                cap,
                start:ptr.as_ptr(),
                end:if(cap==0){
                    ptr.as_ptr()
                }else{
                    ptr.as_ptr().offset(len as isize)
                }
            }
        }
    }
    //-----------------
}
impl<T> Drop for MVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            //取出元素
            while let Some(_) = self.pop() {}
            //回收内存
            let align = mem::align_of::<T>();
            let elem_size = mem::size_of::<T>();
            let num_bytes = elem_size * self.cap;
            unsafe {
                let layout = Layout::from_size_align_unchecked(num_bytes, align);
                dealloc(self.ptr.as_ptr() as *mut _, layout)
            }
        }
        println!("内存释放");
    }
}
impl<T> Deref for MVec<T> {
    //切片类型
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}
impl<T> DerefMut for MVec<T> {
    // type Target =  [T];
    //切片类型
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}
struct IntoIter<T> {
    buf: NonNull<T>,
    cap: usize,
    start: *const T,
    end: *const T,
}
impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            for _ in &mut *self {}
            let align = mem::align_of::<T>();
            let elem_size = mem::size_of::<T>();
            let num_bytes = elem_size * self.cap;
            unsafe {
                let layout = Layout::from_size_align_unchecked(num_bytes, align);
                dealloc(self.buf.as_ptr() as *mut _, layout)
            }
        }
    }
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.start = self.start.offset(1);
                Some(ptr::read(self.start))
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
       let len=((self.end as usize-self.start as usize)/mem::size_of::<T>());
        (len,Some(len))
    }
}
impl<T> DoubleEndedIterator for IntoIter<T>{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        }else {
            unsafe {
                //向前回滚一位
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
    }
}
fn main() {
    let mut vec: MVec<i32> = MVec::new();
    unsafe {
        vec.push(1);
        vec.push(2);
        vec.push(3);
        let v = &mut vec[0..1];
        v[0] = 2;
        println!("{:?}", v);
        println!("{}", vec.get(0).unwrap());
        println!("{:?}", vec);
        vec.push(5);
        vec.insert(3, 8);
        vec.push(6);
        vec.remove(0);
        println!("{}", vec.len);
        println!("{:?}", &vec[0..5]);
        println!("{}", vec.pop().unwrap());
        println!("{}", vec.pop().unwrap());
        drop(vec);
    }
}
