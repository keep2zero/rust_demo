mod video {

    #[derive(Debug)]
    pub struct VideoPlay {
        pub vp_name: String,
        pub vp_play: String,
    }

  
    #[derive(Debug)]
    pub struct Video {
        pub v_name: String,
        pub v_subname: String,
        pub v_image: String,
        pub v_play: VideoPlay,
    }




    #[derive(Debug)]
    pub struct VideoList {
        pub vl_name: String,
        pub vl_list: Vec<Video>
    }

    pub fn change_obj(video: Video) {
        //video.v_name = "hello change".to_string(); //throw exception
        println!("{:?}", video);
    }

    pub fn change_obj_return(mut video: Video) -> Video {
        video.v_name = "change Name".to_string();
        video
    }

    pub fn change_mut_obj(mut video: Video) {
        video.v_name = "hello change".to_string();
        println!("{:?}", video);
    }

    pub fn change_refobj(video: &Video) {
        println!("{:?}", video);
    }


    pub fn change_refobj_return(video: &Video) -> &Video{
        println!("{:?}", video);

        video
    }

    pub fn change_refmut_obj(video: &mut Video, i: u32) {
        video.v_play.vp_name = format!("name: {}", i);
        // println!("{:?}", video);
    }




    pub fn change_list(video: &mut Video) {
            video.v_name = String::from("value");
    }
}

#[cfg(test)]
mod tests {

    use std::{thread, borrow::{Borrow, BorrowMut}};

    use crate::{
        video::{Video, VideoPlay, VideoList},
        *,
    };

    #[test]
    fn test_borrow() {
        let video = Video {
            v_name: "hello".to_owned(),
            v_subname: "test".to_owned(),
            v_image: "".to_owned(),
            v_play: VideoPlay {
                vp_name: "".to_owned(),
                vp_play: "".to_owned(),
            },
        };

        video::change_obj(video);

        for i in [..100] {
            //   video.v_play.vp_name = format!("{:?}", i);
            //video::change_obj(video); //throw exception; because it droped at the first was runned;
            //    change_refobj(&video); // it's normal. it can't change the video fields;

            //    change_refmut_obj(&mut video);
        }

        // println!("{:?}", video); //throw exception
    }

    #[test]
    fn test_borrow_return() {
        let video = Video {
            v_name: "hello".to_owned(),
            v_subname: "test".to_owned(),
            v_image: "".to_owned(),
            v_play: VideoPlay {
                vp_name: "".to_owned(),
                vp_play: "".to_owned(),
            },
        };

        //  video::change_obj_return(video);

        //println!("{:?}", video); // throw exception.

        let video = video::change_obj_return(video);

        println!("{:?}", video);
    }

    #[test]
    fn test_mut() {
        let video = Video {
            v_name: "hello".to_owned(),
            v_subname: "test".to_owned(),
            v_image: "".to_owned(),
            v_play: VideoPlay {
                vp_name: "".to_owned(),
                vp_play: "".to_owned(),
            },
        };

        //函数的传递值为mut， 则在函数声明的参数中加入Mut即可， 不需要在对象上设置Mut.

        video::change_mut_obj(video);

        // println!("{:?}", video);
    }

    #[test]
    fn test_ref() {
        let video = Video {
            v_name: "hello".to_owned(),
            v_subname: "test".to_owned(),
            v_image: "".to_owned(),
            v_play: VideoPlay {
                vp_name: "".to_owned(),
                vp_play: "".to_owned(),
            },
        };
        for _ in 1..100 {
            video::change_refobj(&video);
        }

        println!("{:?}", video);
    }

    #[test]
    fn test_refmut() {
        //如果函数传递的是mut的引用， 则必须设置对象为mut。

        let mut video = Video {
            v_name: "hello".to_owned(),
            v_subname: "test".to_owned(),
            v_image: "".to_owned(),
            v_play: VideoPlay {
                vp_name: "".to_owned(),
                vp_play: "".to_owned(),
            },
        };
        for i in 1..101 {
            video::change_refmut_obj(&mut video, i);
        }

        println!("{:?}", video);
    }


    #[test]
    fn test_refobj_return() {
        let video = Video {
            v_name: "hello".to_owned(),
            v_subname: "test".to_owned(),
            v_image: "".to_owned(),
            v_play: VideoPlay {
                vp_name: "".to_owned(),
                vp_play: "".to_owned(),
            },
        };
        for _ in 1..100 {
           let video = video::change_refobj_return(&video);
           println!("{:?}", video);
        }

        println!("{:?}", video);
    }



   fn buildList() -> VideoList{

    let mut list = VideoList{
        vl_list:vec!(),
        vl_name: "list".to_owned()
         
    };

    for _ in 1..100 {
        let video = Video {
            v_name: "hello".to_owned(),
            v_subname: "test".to_owned(),
            v_image: "".to_owned(),
            v_play: VideoPlay {
                vp_name: "".to_owned(),
                vp_play: "".to_owned(),
            },
        };

        list.vl_list.push(video);

     }

     list

   }



   fn change_list(vs: &mut Vec<Video>) {
     for it in vs {
        it.v_name = "uuuuuuu".to_string();
     }
   }

   fn change_item(uname: String)-> String {
    // it.v_name = "thread".to_string();
    "chagne_item".to_owned()
   }

    #[test]
    fn test_list() {


        let mut  list = buildList();
        

        let   listv = &mut list.vl_list;


        // change_list(listv);
        let mut i = 0;
        for   it in listv {
           let istr = it.v_name.clone();
           let j = thread::spawn(move || {
                // change_item();
                change_item(istr)
            }).join().unwrap();
            it.v_name = format!("{}{}", j, i+1);
            i+=1;
    
        }

        // listv[0].v_name = "helall".to_owned();
        // listv[1].v_name = "helall2".to_owned();


        println!("{:?}", list);

    }
}
