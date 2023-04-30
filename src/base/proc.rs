mod Proc {

    use hashbrown::HashMap;
    use kanal::{
        bounded_async as kanalsB, unbounded_async as kanals, AsyncReceiver as KanalRx,
        AsyncSender as KanalTx,
    };
    use tokio::sync::broadcast::{
        channel as bc_channel, Receiver as BroadCastRx, Sender as BroadCastTx,
    };
    use tokio::sync::watch::{channel as w_channel, Receiver as WatchRx, Sender as WatchTx};
    use tokio::sync::Barrier;
    use tokio::sync::OnceCell;

    use tokio::runtime::{Builder, Handle as RtHandle, Runtime};
    use tokio::task::{spawn, AbortHandle, JoinHandle, JoinSet};
    use tokio::task_local;
    use tokio::time::{Duration, Instant};

    //  Thread and Task profiling
    pub struct Matrisync<T> {
        id: usize,
        role: MatricesRole,
        rt: RT,
        state: RunMode,
        idle: bool,
        j_handle: Option<JoinHandle<T>>,
        a_handle: Option<AbortHandle>,
        t_handle: Option<RtHandle>,
    }

    enum MatricesRole {
        Guest,
        Component,
        Ecosystem,
        TopProc,
    }

    enum RT {
        Compendium,
        Space,
        Ascension,
        Matrices,
    }

    enum RunType {
        Thread,
        Task,
    }

    enum RunMode {
        Init,
        Run,
        Diag,
        Stop,
        End,
    }

    //  Sync and Messaging /////////////////////////////////////////////
    type Kanals = (KanalRx<Msg>, HashMap<usize, KanalTx<Msg>>);
    type Kanal = (KanalTx<Msg>, KanalRx<Msg>);

    //    >---> Tokio  MPMC
    //    >---> Watch  SPMC

    enum Msg {
        Push,
        Pull,
        Poll,
    }

    //  Thread and Task Running ////////////////////////////////////////////////
    trait Runner {
        fn get_name(self) -> String;
        fn build(self) -> Result<Runtime, ()>;
        fn get_handle(self) -> RtHandle;
    }

    impl Runner for RT {
        fn get_name(self) -> String {
            match self {
                RT::Compendium => String::from("Compendium"),
                RT::Space => String::from("Space"),
                RT::Ascension => String::from("Ascension"),
                RT::Matrices => String::from("Matrices"),
            }
        }
        fn build(self) -> Result<Runtime, ()> {
            Builder::new_current_thread()
                .thread_name(self.get_name())
                .on_thread_start(|| ()) // TODO!
                .on_thread_stop(|| ()) // TODO!
                .on_thread_park(|| ()) // TODO!
                .on_thread_unpark(|| ()) // TODO!
                .enable_time()
                .enable_io()
                .start_paused(true)
                .build()
                .map_err(|_| ()) // TODO!
                .map(|rt| rt) // TODO!
        }
        fn get_handle(self) -> RtHandle {
            unimplemented!()
            // match self {
            // RT::Compendium => ,
            // RT::Space => ,
            // RT::Ascension => ,
            // RT::Matrices => ,
            // };
        }
    }

    //////////////////////////////////////////////////
    // Go time!
    /////////////////
    mod Engine {
        use super::*;

        pub fn get_ready() -> {
            let (tx_watch, rx_watch) = kanals();
            let mut proc_map = HashMap::new();
            loop {}
        }

        pub fn get_set() -> HashSet<()> {
            HashSet::new()
        }

        macro_rules! GO {
            () => {};
        }
    }
}
