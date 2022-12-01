package com.veilid.veilid_core_android_tests;

import androidx.appcompat.app.AppCompatActivity;
import android.content.Context;
import android.os.Bundle;

public class MainActivity extends AppCompatActivity {

    static {
        System.loadLibrary("veilid_core");
    }

    private static native void run_tests(Context context);

    private Thread testThread;

    class TestThread extends Thread {
        private Context context;

        TestThread(Context context) {
            this.context = context;
        }

        public void run() {
        }
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        run_tests(this.context);
    }
}
