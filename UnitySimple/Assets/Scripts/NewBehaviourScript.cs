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
        var addRet = NativeMethods.my_add(1, 2);
        Debug.Log($"native method add(1,2) ={addRet}");
    }

    public void TestSubFunc()
    {
        var subRet = NativeMethods.my_sub(1, 2);
        Debug.Log($"native method sub(1,2) ={subRet}");
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
    }
}