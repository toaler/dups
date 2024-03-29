import React from 'react';

function StagingTab({actions}) { // Use the correct prop name here
    return (
        <div>
            <p>The staging view is used for preparing changes to be carried out on the filesystem.</p>
            <table>
                <thead>
                <tr>
                    <th style={{textAlign: "left"}}>Action</th>
                    <th style={{textAlign: "left"}}>Resource</th>
                    <th style={{textAlign: "right"}}>Bytes</th>
                </tr>
                </thead>
                <tbody>
                {actions.map((actionObj, index) => (
                    <tr key={index}>
                        <td>{actionObj.action}</td>
                        <td>{actionObj.path}</td>
                        <td style={{textAlign: "right"}}>{actionObj.bytes.toLocaleString("en-US")}</td>
                    </tr>
                ))}
                </tbody>
            </table>
        </div>
    );
}

export default StagingTab;