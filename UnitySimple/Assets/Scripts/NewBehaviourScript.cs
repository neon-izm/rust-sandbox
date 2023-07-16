using System.Collections;
using System.Collections.Generic;
using CsBindgen;
using UnityEngine;

public class NewBehaviourScript : MonoBehaviour
{
    // Start is called before the first frame update
    void Start()
    {
    }

    public void TestAddFunc()
    {
        var ret = NativeMethods.my_add(1, 2);
        Debug.Log($"native method add(1,2) ={ret}");
    }

    public void TestSubFunc()
    {
        var ret = NativeMethods.my_sub(1, 2);
        Debug.Log($"native method sub(1,2) ={ret}");
    }

    public void TestMultiFunc()
    {
        var ret = NativeMethods.my_multiply(5, 6);
        Debug.Log($"native method multi(5,6) ={ret}");
    }

    public void TestDivFunc()
    {
        var ret = NativeMethods.my_divide(36, 12);
        Debug.Log($"native method div(36,12) ={ret}");
    }

    // Update is called once per frame
    void Update()
    {
        if (Input.GetKeyDown(KeyCode.A))
        {
            TestAddFunc();
        }

        if (Input.GetKeyDown(KeyCode.B))
        {
            TestSubFunc();
        }

        if (Input.GetKeyDown(KeyCode.C))
        {
            TestMultiFunc();
        }

        if (Input.GetKeyDown(KeyCode.D))
        {
            TestDivFunc();
        }
    }
}