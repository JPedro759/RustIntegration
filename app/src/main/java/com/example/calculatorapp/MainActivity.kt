package com.example.calculatorapp

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.example.calculatorapp.ui.theme.CalculatorAppTheme

data class DivisionResponse(val value: Float, val status: Boolean)

class MainActivity : ComponentActivity() {

    //<RustJNI>
    // auto-generated code
           
    external fun sayHello(): String

    external fun sumIntNumbers(a: Int, b: Int): Int

    external fun subtract(a: Int, b: Int): Int

    external fun multiply(a: Int, b: Int): Int

    external fun divide(dividend: Float, divisor: Float): Any
           
    init { System.loadLibrary("my_rust_lib") }
           
    //</RustJNI>

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            CalculatorAppTheme {
                Column(modifier = Modifier.fillMaxSize()) {
                    val modifier = Modifier.padding(top = 35.dp, start = 30.dp)

                    Text(
                        text = sayHello(),
                        modifier = Modifier.padding(top = 75.dp, start = 30.dp)
                    )

                    Text(text = "SUM = ${sumIntNumbers(5, 7)}", modifier = modifier)
                    Text(text = "SUB = ${subtract(10, 4)}", modifier = modifier)
                    Text(text = "MULT = ${multiply(25, 73)}", modifier = modifier)

                    val division = divide(5F, 3F) as DivisionResponse

                    val result =
                        if (division.status) division.value
                        else "ERROR: Division by zero!"

                    Text(text = "DIV = $result", modifier = modifier)
                }
            }
        }
    }
}